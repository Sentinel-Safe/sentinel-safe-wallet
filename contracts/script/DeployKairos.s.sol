// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.30;

import "forge-std/Script.sol";
import "../src/SafeRoleGuard.sol";

interface ISafeProxyFactory {
    function createProxy(
        address singleton,
        bytes memory data
    ) external returns (address);
}

interface ISafeSetup {
    function setup(
        address[] calldata _owners,
        uint256 _threshold,
        address to,
        bytes calldata data,
        address fallbackHandler,
        address paymentToken,
        uint256 payment,
        address payable paymentReceiver
    ) external;
}

/**
 * @title DeployKairos
 * @notice Deployment script for Kaia Kairos Testnet ONLY
 * @dev Deploys Safe multi-sig with SafeRoleGuard for 2-human-3-AI composition
 */
contract DeployKairos is Script {
    // Kaia Kairos Testnet Safe Infrastructure
    // These are the official Safe addresses on Kaia Kairos testnet
    address constant SAFE_SINGLETON = 0xfb1bffC9d739B8D520DaF37dF666da4C687191EA;
    address constant SAFE_PROXY_FACTORY = 0xC22834581EbC8527d974F8a1c97E1bEA4EF910BC;
    address constant SAFE_FALLBACK_HANDLER = 0x017062a1dE2FE6b99BE3d9d37841FeD19F573804;

    // Required threshold for super majority
    uint256 constant THRESHOLD = 4; // 4-of-5 signatures required

    function run() external {
        // Load environment variables
        uint256 deployerPrivateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        
        // Load signer addresses from environment
        address human1 = vm.envAddress("HUMAN_SIGNER_1");
        address human2 = vm.envAddress("HUMAN_SIGNER_2");
        address aiCfo = vm.envAddress("AI_CFO_ADDRESS");
        address aiSecurity = vm.envAddress("AI_SECURITY_ADDRESS");
        address aiAnalyst = vm.envAddress("AI_ANALYST_ADDRESS");

        console.log("\n=== Kaia Kairos Testnet Deployment ===");
        console.log("Chain ID: 1001");
        console.log("RPC: https://public-en.kairos.node.kaia.io");
        console.log("Deployer:", vm.addr(deployerPrivateKey));

        vm.startBroadcast(deployerPrivateKey);

        // 1. Deploy Safe with 5 owners
        address safe = deploySafe(
            human1,
            human2,
            aiCfo,
            aiSecurity,
            aiAnalyst
        );
        
        // 2. Deploy SafeRoleGuard
        SafeRoleGuard guard = deployRoleGuard(
            safe,
            human1,
            human2,
            aiCfo,
            aiSecurity,
            aiAnalyst
        );

        // 3. Log deployment info
        logDeploymentInfo(safe, address(guard));
        
        // 4. Save deployment addresses
        saveDeploymentAddresses(safe, address(guard));

        vm.stopBroadcast();

        console.log("\nIMPORTANT: To activate the guard, execute this through the Safe:");
        console.log("   Target: %s", safe);
        console.log("   Function: setGuard(address)");
        console.log("   Parameter: %s", address(guard));
        console.log("\nDeployment complete! Addresses saved to contracts/.env.deployed");
    }

    function deploySafe(
        address human1,
        address human2,
        address aiCfo,
        address aiSecurity,
        address aiAnalyst
    ) internal returns (address) {
        console.log("\nDeploying Safe multi-sig...");
        
        // Prepare owners array
        address[] memory owners = new address[](5);
        owners[0] = human1;
        owners[1] = human2;
        owners[2] = aiCfo;
        owners[3] = aiSecurity;
        owners[4] = aiAnalyst;

        // Encode Safe initialization
        bytes memory initializer = abi.encodeWithSelector(
            ISafeSetup.setup.selector,
            owners,
            THRESHOLD,
            address(0), // to
            new bytes(0), // data
            SAFE_FALLBACK_HANDLER,
            address(0), // paymentToken
            0, // payment
            payable(address(0)) // paymentReceiver
        );

        // Deploy Safe proxy
        address safe = ISafeProxyFactory(SAFE_PROXY_FACTORY).createProxy(
            SAFE_SINGLETON,
            initializer
        );
        
        console.log("Safe deployed at:", safe);
        return safe;
    }

    function deployRoleGuard(
        address safe,
        address human1,
        address human2,
        address aiCfo,
        address aiSecurity,
        address aiAnalyst
    ) internal returns (SafeRoleGuard) {
        console.log("\nDeploying SafeRoleGuard...");
        
        // Prepare human owners
        address[] memory humanOwners = new address[](2);
        humanOwners[0] = human1;
        humanOwners[1] = human2;

        // Prepare AI owners
        address[] memory aiOwners = new address[](3);
        aiOwners[0] = aiCfo;
        aiOwners[1] = aiSecurity;
        aiOwners[2] = aiAnalyst;

        // Deploy guard
        SafeRoleGuard guard = new SafeRoleGuard(safe, humanOwners, aiOwners);
        console.log("SafeRoleGuard deployed at:", address(guard));
        
        return guard;
    }

    function logDeploymentInfo(address safe, address guard) internal view {
        console.log("\n=== Deployment Summary ===");
        console.log("Safe Address:", safe);
        console.log("Guard Address:", guard);
        console.log("Threshold: %d of 5", THRESHOLD);
        console.log("Network: Kaia Kairos Testnet");
        console.log("Block Explorer: https://kairos.kaiascope.com/");
    }

    function saveDeploymentAddresses(address safe, address guard) internal {
        string memory timestamp = vm.toString(block.timestamp);
        
        string memory content = string(abi.encodePacked(
            "# Kaia Kairos Testnet Deployment\n",
            "# Timestamp: ", timestamp, "\n",
            "# Block: ", vm.toString(block.number), "\n\n",
            "SAFE_PROXY_ADDRESS=", vm.toString(safe), "\n",
            "SAFE_ROLE_GUARD_ADDRESS=", vm.toString(guard), "\n",
            "\n# Safe Infrastructure (Kairos)\n",
            "SAFE_SINGLETON=", vm.toString(SAFE_SINGLETON), "\n",
            "SAFE_PROXY_FACTORY=", vm.toString(SAFE_PROXY_FACTORY), "\n",
            "SAFE_FALLBACK_HANDLER=", vm.toString(SAFE_FALLBACK_HANDLER), "\n"
        ));
        
        vm.writeFile("contracts/.env.deployed", content);
    }
}
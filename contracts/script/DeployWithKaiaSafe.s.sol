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
 * @title DeployWithKaiaSafe
 * @notice Deployment script for setting up Kaia Safe with RoleGuard
 * @dev Uses existing Kaia Safe infrastructure with custom guard
 */
contract DeployWithKaiaSafe is Script {
    // Kaia Safe addresses (실제 주소 아님; kairos 테스트넷 주소임)
    address constant SAFE_SINGLETON =
        0xfb1bffC9d739B8D520DaF37dF666da4C687191EA;
    address constant SAFE_PROXY_FACTORY =
        0xC22834581EbC8527d974F8a1c97E1bEA4EF910BC;
    address constant SAFE_FALLBACK_HANDLER =
        0x017062a1dE2FE6b99BE3d9d37841FeD19F573804;

    // Owner configuration (update before deployment)
    address constant HUMAN_1 =
        address(0x1111111111111111111111111111111111111111);
    address constant HUMAN_2 =
        address(0x2222222222222222222222222222222222222222);
    address constant AI_CFO =
        address(0x3333333333333333333333333333333333333333);
    address constant AI_SECURITY =
        address(0x4444444444444444444444444444444444444444);
    address constant AI_ANALYST =
        address(0x5555555555555555555555555555555555555555);

    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        // Prepare owners array
        address[] memory owners = new address[](5);
        owners[0] = HUMAN_1;
        owners[1] = HUMAN_2;
        owners[2] = AI_CFO;
        owners[3] = AI_SECURITY;
        owners[4] = AI_ANALYST;

        // Deploy Safe using factory
        console.log("Deploying Kaia Safe...");
        address safe = _deploySafe(owners, 4); // 4-of-5 threshold
        console.log("Safe deployed at:", safe);

        // Deploy RoleGuard
        console.log("Deploying SafeRoleGuard...");
        address[] memory humanOwners = new address[](2);
        humanOwners[0] = HUMAN_1;
        humanOwners[1] = HUMAN_2;

        address[] memory aiOwners = new address[](3);
        aiOwners[0] = AI_CFO;
        aiOwners[1] = AI_SECURITY;
        aiOwners[2] = AI_ANALYST;

        SafeRoleGuard guard = new SafeRoleGuard(safe, humanOwners, aiOwners);
        console.log("SafeRoleGuard deployed at:", address(guard));

        // Set guard on Safe (needs to be done through Safe transaction)
        console.log("\n=== IMPORTANT ===");
        console.log(
            "To activate the guard, execute this transaction through the Safe:"
        );
        console.log("Target:", safe);
        console.log("Function: setGuard(address)");
        console.log("Parameter:", address(guard));

        _logDeploymentSummary(safe, address(guard));

        vm.stopBroadcast();
    }

    function _deploySafe(
        address[] memory owners,
        uint256 threshold
    ) private returns (address) {
        bytes memory initializer = abi.encodeWithSelector(
            ISafeSetup.setup.selector,
            owners,
            threshold,
            address(0), // to
            new bytes(0), // data
            SAFE_FALLBACK_HANDLER,
            address(0), // paymentToken
            0, // payment
            payable(address(0)) // paymentReceiver
        );

        return
            ISafeProxyFactory(SAFE_PROXY_FACTORY).createProxy(
                SAFE_SINGLETON,
                initializer
            );
    }

    function _logDeploymentSummary(address safe, address guard) private pure {
        console.log("\n=== Deployment Summary ===");
        console.log("Safe Address:", safe);
        console.log("Guard Address:", guard);
        console.log("\n=== Configuration ===");
        console.log("Required Signatures: 4 of 5");
        console.log("Human Owners: 2");
        console.log("AI Owners: 3");
        console.log("\n=== Owner Addresses ===");
        console.log("Human 1:", HUMAN_1);
        console.log("Human 2:", HUMAN_2);
        console.log("AI CFO:", AI_CFO);
        console.log("AI Security:", AI_SECURITY);
        console.log("AI Analyst:", AI_ANALYST);
    }
}

/**
 * @title DeployKaiaTestnet
 * @notice Deployment for Kaia Kairos testnet with environment variables
 */
contract DeployKaiaTestnet is Script {
    // Kaia Kairos testnet addresses
    address constant SAFE_SINGLETON =
        0xfb1bffC9d739B8D520DaF37dF666da4C687191EA;
    address constant SAFE_PROXY_FACTORY =
        0xC22834581EbC8527d974F8a1c97E1bEA4EF910BC;
    address constant SAFE_FALLBACK_HANDLER =
        0x017062a1dE2FE6b99BE3d9d37841FeD19F573804;

    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");

        // Read owner addresses from environment
        address human1 = vm.envAddress("HUMAN_1_ADDRESS");
        address human2 = vm.envAddress("HUMAN_2_ADDRESS");
        address aiCfo = vm.envAddress("AI_CFO_ADDRESS");
        address aiSecurity = vm.envAddress("AI_SECURITY_ADDRESS");
        address aiAnalyst = vm.envAddress("AI_ANALYST_ADDRESS");

        vm.startBroadcast(deployerPrivateKey);

        // Prepare owners array
        address[] memory owners = new address[](5);
        owners[0] = human1;
        owners[1] = human2;
        owners[2] = aiCfo;
        owners[3] = aiSecurity;
        owners[4] = aiAnalyst;

        // Deploy Safe
        console.log("Deploying Safe on Kaia Kairos testnet...");
        bytes memory initializer = abi.encodeWithSelector(
            ISafeSetup.setup.selector,
            owners,
            4, // threshold
            address(0),
            new bytes(0),
            SAFE_FALLBACK_HANDLER,
            address(0),
            0,
            payable(address(0))
        );

        address safe = ISafeProxyFactory(SAFE_PROXY_FACTORY).createProxy(
            SAFE_SINGLETON,
            initializer
        );
        console.log("Safe deployed at:", safe);

        // Deploy guard
        address[] memory humanOwners = new address[](2);
        humanOwners[0] = human1;
        humanOwners[1] = human2;

        address[] memory aiOwners = new address[](3);
        aiOwners[0] = aiCfo;
        aiOwners[1] = aiSecurity;
        aiOwners[2] = aiAnalyst;

        SafeRoleGuard guard = new SafeRoleGuard(safe, humanOwners, aiOwners);
        console.log("SafeRoleGuard deployed at:", address(guard));

        // Write deployment addresses
        string memory deploymentInfo = string(
            abi.encodePacked(
                "SAFE_ADDRESS=",
                vm.toString(safe),
                "\n",
                "SAFE_ROLE_GUARD_ADDRESS=",
                vm.toString(address(guard)),
                "\n"
            )
        );

        vm.writeFile(".env.contracts", deploymentInfo);
        console.log("\nDeployment addresses written to .env.contracts");

        vm.stopBroadcast();
    }
}

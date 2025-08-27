// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import "../src/SafeRoleGuard.sol";
import "../src/interfaces/ISafe.sol";

// Mock Safe contract for testing
contract MockSafe is ISafe {
    address[] private owners;
    uint256 private threshold;
    address public guard;
    uint256 public nonce;
    
    constructor(address[] memory _owners, uint256 _threshold) {
        owners = _owners;
        threshold = _threshold;
    }
    
    function getOwners() external view override returns (address[] memory) {
        return owners;
    }
    
    function isOwner(address owner) external view override returns (bool) {
        for (uint i = 0; i < owners.length; i++) {
            if (owners[i] == owner) return true;
        }
        return false;
    }
    
    function getThreshold() external view override returns (uint256) {
        return threshold;
    }
    
    function addOwnerWithThreshold(address owner, uint256 _threshold) external override {
        owners.push(owner);
        threshold = _threshold;
    }
    
    function removeOwner(address, address owner, uint256 _threshold) external override {
        address[] memory newOwners = new address[](owners.length - 1);
        uint j = 0;
        for (uint i = 0; i < owners.length; i++) {
            if (owners[i] != owner) {
                newOwners[j++] = owners[i];
            }
        }
        owners = newOwners;
        threshold = _threshold;
    }
    
    function swapOwner(address, address oldOwner, address newOwner) external override {
        for (uint i = 0; i < owners.length; i++) {
            if (owners[i] == oldOwner) {
                owners[i] = newOwner;
                break;
            }
        }
    }
    
    function changeThreshold(uint256 _threshold) external override {
        threshold = _threshold;
    }
    
    function setGuard(address _guard) external override {
        guard = _guard;
    }
    
    function execTransaction(
        address,
        uint256,
        bytes calldata,
        uint8,
        uint256,
        uint256,
        uint256,
        address,
        address payable,
        bytes memory
    ) external payable override returns (bool) {
        return true;
    }
    
    function getTransactionHash(
        address,
        uint256,
        bytes calldata,
        uint8,
        uint256,
        uint256,
        uint256,
        address,
        address,
        uint256
    ) external pure override returns (bytes32) {
        return bytes32(0);
    }
}

contract SafeRoleGuardTest is Test {
    SafeRoleGuard public guard;
    MockSafe public safe;
    
    // Test accounts
    address human1 = makeAddr("human1");
    address human2 = makeAddr("human2");
    address ai1 = makeAddr("ai1");
    address ai2 = makeAddr("ai2");
    address ai3 = makeAddr("ai3");
    address newHuman = makeAddr("newHuman");
    address newAI = makeAddr("newAI");
    address attacker = makeAddr("attacker");

    function setUp() public {
        // Create Safe with correct owners
        address[] memory owners = new address[](5);
        owners[0] = human1;
        owners[1] = human2;
        owners[2] = ai1;
        owners[3] = ai2;
        owners[4] = ai3;
        
        safe = new MockSafe(owners, 4);
        
        // Create guard
        address[] memory humanOwners = new address[](2);
        humanOwners[0] = human1;
        humanOwners[1] = human2;
        
        address[] memory aiOwners = new address[](3);
        aiOwners[0] = ai1;
        aiOwners[1] = ai2;
        aiOwners[2] = ai3;
        
        guard = new SafeRoleGuard(address(safe), humanOwners, aiOwners);
    }

    function testInitialSetup() public view {
        // Check role assignments
        assertTrue(guard.isHuman(human1));
        assertTrue(guard.isHuman(human2));
        assertTrue(guard.isAI(ai1));
        assertTrue(guard.isAI(ai2));
        assertTrue(guard.isAI(ai3));
        
        // Check composition
        (uint256 humanCount, uint256 aiCount) = guard.getRoleComposition();
        assertEq(humanCount, 2);
        assertEq(aiCount, 3);
        
        // Check valid composition
        assertTrue(guard.isValidComposition());
    }

    function testCheckTransactionNormalCall() public {
        // Normal transaction should pass
        vm.prank(address(safe));
        guard.checkTransaction(
            address(0x1234), // to
            1 ether, // value
            "", // data
            0, // operation
            0, // safeTxGas
            0, // baseGas
            0, // gasPrice
            address(0), // gasToken
            payable(address(0)), // refundReceiver
            "", // signatures
            human1 // msgSender
        );
    }

    function testCheckTransactionBlockAddOwner() public {
        bytes memory addOwnerData = abi.encodeWithSelector(
            ISafe.addOwnerWithThreshold.selector,
            attacker,
            4
        );
        
        vm.prank(address(safe));
        vm.expectRevert(SafeRoleGuard.UnauthorizedOwnerChange.selector);
        guard.checkTransaction(
            address(safe), // to (self)
            0, // value
            addOwnerData, // data
            0, // operation
            0, // safeTxGas
            0, // baseGas
            0, // gasPrice
            address(0), // gasToken
            payable(address(0)), // refundReceiver
            "", // signatures
            human1 // msgSender
        );
    }

    function testCheckTransactionBlockRemoveOwner() public {
        bytes memory removeOwnerData = abi.encodeWithSelector(
            ISafe.removeOwner.selector,
            human1,
            human2,
            3
        );
        
        vm.prank(address(safe));
        vm.expectRevert(SafeRoleGuard.UnauthorizedOwnerChange.selector);
        guard.checkTransaction(
            address(safe), // to (self)
            0, // value
            removeOwnerData, // data
            0, // operation
            0, // safeTxGas
            0, // baseGas
            0, // gasPrice
            address(0), // gasToken
            payable(address(0)), // refundReceiver
            "", // signatures
            human1 // msgSender
        );
    }

    function testCheckTransactionAllowValidSwap() public {
        // Swap human with human (should be allowed)
        bytes memory swapData = abi.encodeWithSelector(
            ISafe.swapOwner.selector,
            human1, // prevOwner
            human2, // oldOwner
            newHuman // newOwner
        );
        
        // First perform the actual swap in the safe
        safe.swapOwner(human1, human2, newHuman);
        
        vm.prank(address(safe));
        guard.checkTransaction(
            address(safe), // to (self)
            0, // value
            swapData, // data
            0, // operation
            0, // safeTxGas
            0, // baseGas
            0, // gasPrice
            address(0), // gasToken
            payable(address(0)), // refundReceiver
            "", // signatures
            human1 // msgSender
        );
        
        // Check role was updated
        assertFalse(guard.isHuman(human2));
        assertTrue(guard.isHuman(newHuman));
    }

    function testCheckTransactionBlockInvalidThreshold() public {
        bytes memory changeThresholdData = abi.encodeWithSelector(
            ISafe.changeThreshold.selector,
            3 // Invalid threshold (should be 4)
        );
        
        vm.prank(address(safe));
        vm.expectRevert(SafeRoleGuard.InvalidThreshold.selector);
        guard.checkTransaction(
            address(safe), // to (self)
            0, // value
            changeThresholdData, // data
            0, // operation
            0, // safeTxGas
            0, // baseGas
            0, // gasPrice
            address(0), // gasToken
            payable(address(0)), // refundReceiver
            "", // signatures
            human1 // msgSender
        );
    }

    function testCheckAfterExecution() public {
        bytes32 txHash = keccak256("test");
        
        vm.prank(address(safe));
        guard.checkAfterExecution(txHash, true);
        
        // Should emit event (check in traces)
    }

    function testOnlySafeCanCallCheckTransaction() public {
        vm.prank(attacker);
        vm.expectRevert(SafeRoleGuard.NotSafe.selector);
        guard.checkTransaction(
            address(0), 0, "", 0, 0, 0, 0, 
            address(0), payable(address(0)), "", address(0)
        );
    }

    function testOnlySafeCanCallCheckAfterExecution() public {
        vm.prank(attacker);
        vm.expectRevert(SafeRoleGuard.NotSafe.selector);
        guard.checkAfterExecution(bytes32(0), true);
    }

    function testSwapAIWithAI() public {
        bytes memory swapData = abi.encodeWithSelector(
            ISafe.swapOwner.selector,
            ai1, // prevOwner
            ai2, // oldOwner
            newAI // newOwner
        );
        
        // First perform the actual swap in the safe
        safe.swapOwner(ai1, ai2, newAI);
        
        vm.prank(address(safe));
        guard.checkTransaction(
            address(safe),
            0,
            swapData,
            0, 0, 0, 0,
            address(0),
            payable(address(0)),
            "",
            human1
        );
        
        // Check role was updated
        assertFalse(guard.isAI(ai2));
        assertTrue(guard.isAI(newAI));
    }

    function testInvalidCompositionAfterManualChange() public {
        // Manually break the composition
        safe.removeOwner(human1, human2, 4);
        
        // Guard should detect invalid composition
        assertFalse(guard.isValidComposition());
        
        vm.prank(address(safe));
        vm.expectRevert(SafeRoleGuard.InvalidOwnerCount.selector);
        guard.checkTransaction(
            address(0x1234),
            0,
            "",
            0, 0, 0, 0,
            address(0),
            payable(address(0)),
            "",
            ai1
        );
    }

    function testConstants() public view {
        assertEq(guard.REQUIRED_THRESHOLD(), 4);
        assertEq(guard.REQUIRED_HUMANS(), 2);
        assertEq(guard.REQUIRED_AI(), 3);
        assertEq(guard.TOTAL_OWNERS(), 5);
    }
}
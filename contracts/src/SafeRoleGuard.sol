// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.30;

import "./interfaces/ISafe.sol";

/**
 * @title SafeRoleGuard
 * @notice Guard contract for Kaia Safe that enforces 2 humans + 3 AI agents composition
 * @dev Implements the Guard interface from Gnosis Safe to intercept and validate transactions
 */
contract SafeRoleGuard {
    // Constants
    uint256 public constant REQUIRED_THRESHOLD = 4;
    uint256 public constant REQUIRED_HUMANS = 2;
    uint256 public constant REQUIRED_AI = 3;
    uint256 public constant TOTAL_OWNERS = 5;

    // State
    ISafe public immutable safe;
    mapping(address => bool) public isHuman;
    mapping(address => bool) public isAI;
    
    // Events
    event OwnerRoleSet(address indexed owner, bool isHuman);
    event GuardCheckPerformed(bytes32 indexed txHash, bool allowed);
    event OwnerSwapValidated(address indexed oldOwner, address indexed newOwner);

    // Errors
    error InvalidThreshold();
    error InvalidOwnerCount();
    error InvalidOwnerComposition();
    error UnauthorizedOwnerChange();
    error NotSafe();

    modifier onlySafe() {
        if (msg.sender != address(safe)) revert NotSafe();
        _;
    }

    /**
     * @notice Initialize the guard with Safe address and owner roles
     * @param _safe Address of the Kaia Safe contract
     * @param _humanOwners Array of human owner addresses (must be exactly 2)
     * @param _aiOwners Array of AI agent addresses (must be exactly 3)
     */
    constructor(
        address _safe,
        address[] memory _humanOwners,
        address[] memory _aiOwners
    ) {
        require(_safe != address(0), "Invalid safe address");
        require(_humanOwners.length == REQUIRED_HUMANS, "Must have exactly 2 human owners");
        require(_aiOwners.length == REQUIRED_AI, "Must have exactly 3 AI owners");

        safe = ISafe(_safe);

        // Set human owners
        for (uint256 i = 0; i < _humanOwners.length; i++) {
            require(_humanOwners[i] != address(0), "Invalid human owner");
            require(!isHuman[_humanOwners[i]] && !isAI[_humanOwners[i]], "Duplicate owner");
            
            isHuman[_humanOwners[i]] = true;
            emit OwnerRoleSet(_humanOwners[i], true);
        }

        // Set AI owners
        for (uint256 i = 0; i < _aiOwners.length; i++) {
            require(_aiOwners[i] != address(0), "Invalid AI owner");
            require(!isHuman[_aiOwners[i]] && !isAI[_aiOwners[i]], "Duplicate owner");
            
            isAI[_aiOwners[i]] = true;
            emit OwnerRoleSet(_aiOwners[i], false);
        }
    }

    /**
     * @notice Check transaction before execution (Guard interface)
     * @dev Called by Safe before executing a transaction
     * @param to Destination address
     * @param value Ether value
     * @param data Transaction data
     * @param operation Operation type (0: Call, 1: DelegateCall)
     */
    function checkTransaction(
        address to,
        uint256 value,
        bytes memory data,
        uint8 operation,
        uint256 safeTxGas,
        uint256 baseGas,
        uint256 gasPrice,
        address gasToken,
        address payable refundReceiver,
        bytes memory signatures,
        address msgSender
    ) external onlySafe {
        // Check if this is an owner management transaction
        if (to == address(safe)) {
            _validateOwnerManagementTransaction(data);
        } else {
            // For non-Safe transactions, verify current composition
            _validateCurrentComposition();
        }
    }

    /**
     * @notice Check after execution (Guard interface)
     * @dev Called by Safe after executing a transaction
     */
    function checkAfterExecution(bytes32 txHash, bool success) external onlySafe {
        if (success) {
            // Verify composition is still valid after execution
            _validateCurrentComposition();
        }
        emit GuardCheckPerformed(txHash, success);
    }

    /**
     * @notice Validate owner management transactions
     * @param data Transaction data to parse
     */
    function _validateOwnerManagementTransaction(bytes memory data) private {
        if (data.length < 4) return;
        
        bytes4 selector = _getSelector(data);
        
        // Check for addOwnerWithThreshold
        if (selector == ISafe.addOwnerWithThreshold.selector) {
            revert UnauthorizedOwnerChange();
        }
        
        // Check for removeOwner
        if (selector == ISafe.removeOwner.selector) {
            revert UnauthorizedOwnerChange();
        }
        
        // Check for swapOwner - this is the only allowed operation
        if (selector == ISafe.swapOwner.selector) {
            (,address oldOwner, address newOwner) = abi.decode(
                _slice(data, 4, data.length - 4),
                (address, address, address)
            );
            _validateOwnerSwap(oldOwner, newOwner);
        }
        
        // Check for changeThreshold
        if (selector == ISafe.changeThreshold.selector) {
            uint256 newThreshold = abi.decode(_slice(data, 4, 32), (uint256));
            if (newThreshold != REQUIRED_THRESHOLD) {
                revert InvalidThreshold();
            }
        }
    }

    /**
     * @notice Validate owner swap maintains role composition
     * @param oldOwner Owner being removed
     * @param newOwner Owner being added
     */
    function _validateOwnerSwap(address oldOwner, address newOwner) private {
        // Both must be same type (human-human or AI-AI)
        bool oldIsHuman = isHuman[oldOwner];
        bool oldIsAI = isAI[oldOwner];
        
        require(oldIsHuman || oldIsAI, "Unknown old owner");
        require(!isHuman[newOwner] && !isAI[newOwner], "New owner already registered");
        
        // Update role mappings for the swap
        if (oldIsHuman) {
            delete isHuman[oldOwner];
            isHuman[newOwner] = true;
        } else {
            delete isAI[oldOwner];
            isAI[newOwner] = true;
        }
        
        emit OwnerSwapValidated(oldOwner, newOwner);
    }

    /**
     * @notice Validate current Safe composition
     */
    function _validateCurrentComposition() private view {
        address[] memory owners = safe.getOwners();
        
        if (owners.length != TOTAL_OWNERS) {
            revert InvalidOwnerCount();
        }
        
        uint256 humanCount;
        uint256 aiCount;
        
        for (uint256 i = 0; i < owners.length; i++) {
            if (isHuman[owners[i]]) {
                humanCount++;
            } else if (isAI[owners[i]]) {
                aiCount++;
            }
        }
        
        if (humanCount != REQUIRED_HUMANS || aiCount != REQUIRED_AI) {
            revert InvalidOwnerComposition();
        }
        
        if (safe.getThreshold() != REQUIRED_THRESHOLD) {
            revert InvalidThreshold();
        }
    }

    /**
     * @notice Get the current role composition
     * @return humanCount Number of human owners
     * @return aiCount Number of AI owners
     */
    function getRoleComposition() external view returns (
        uint256 humanCount,
        uint256 aiCount
    ) {
        address[] memory owners = safe.getOwners();
        
        for (uint256 i = 0; i < owners.length; i++) {
            if (isHuman[owners[i]]) {
                humanCount++;
            } else if (isAI[owners[i]]) {
                aiCount++;
            }
        }
    }

    /**
     * @notice Check if composition is valid
     * @return valid Whether the current composition is valid
     */
    function isValidComposition() external view returns (bool valid) {
        try this.getRoleComposition() returns (uint256 humanCount, uint256 aiCount) {
            valid = (humanCount == REQUIRED_HUMANS && aiCount == REQUIRED_AI);
        } catch {
            valid = false;
        }
        
        valid = valid && (safe.getThreshold() == REQUIRED_THRESHOLD);
        valid = valid && (safe.getOwners().length == TOTAL_OWNERS);
    }

    /**
     * @notice Extract function selector from calldata
     */
    function _getSelector(bytes memory data) private pure returns (bytes4) {
        return bytes4(data[0]) | (bytes4(data[1]) >> 8) | (bytes4(data[2]) >> 16) | (bytes4(data[3]) >> 24);
    }

    /**
     * @notice Slice bytes array
     */
    function _slice(bytes memory data, uint256 start, uint256 length) private pure returns (bytes memory) {
        bytes memory result = new bytes(length);
        for (uint256 i = 0; i < length; i++) {
            result[i] = data[start + i];
        }
        return result;
    }
}
// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.19;

/**
 * @title ISafe
 * @notice Interface for Gnosis Safe (Kaia Safe) core functionality
 * @dev Minimal interface for interacting with deployed Safe contracts
 */
interface ISafe {
    function getOwners() external view returns (address[] memory);
    
    function isOwner(address owner) external view returns (bool);
    
    function getThreshold() external view returns (uint256);
    
    function addOwnerWithThreshold(address owner, uint256 _threshold) external;
    
    function removeOwner(address prevOwner, address owner, uint256 _threshold) external;
    
    function swapOwner(address prevOwner, address oldOwner, address newOwner) external;
    
    function changeThreshold(uint256 _threshold) external;
    
    function execTransaction(
        address to,
        uint256 value,
        bytes calldata data,
        uint8 operation,
        uint256 safeTxGas,
        uint256 baseGas,
        uint256 gasPrice,
        address gasToken,
        address payable refundReceiver,
        bytes memory signatures
    ) external payable returns (bool success);
    
    function setGuard(address guard) external;
    
    function getTransactionHash(
        address to,
        uint256 value,
        bytes calldata data,
        uint8 operation,
        uint256 safeTxGas,
        uint256 baseGas,
        uint256 gasPrice,
        address gasToken,
        address refundReceiver,
        uint256 _nonce
    ) external view returns (bytes32);
    
    function nonce() external view returns (uint256);
}
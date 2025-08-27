// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.30;

/**
 * @title IRoleGuard
 * @notice Interface for the RoleGuard contract that enforces owner role composition
 */
interface IRoleGuard {
    /**
     * @notice Validate that an owner replacement maintains the required role composition
     * @param oldOwner Address being removed
     * @param newOwner Address being added
     * @param isOldOwnerHuman Whether the old owner is human
     * @param isNewOwnerHuman Whether the new owner is human
     * @return valid Whether the replacement is valid
     */
    function validateOwnerReplacement(
        address oldOwner,
        address newOwner,
        bool isOldOwnerHuman,
        bool isNewOwnerHuman
    ) external view returns (bool valid);

    /**
     * @notice Check if a role composition is valid (2 humans + 3 AI)
     * @param humanCount Number of human owners
     * @param aiCount Number of AI owners
     * @return valid Whether the composition is valid
     */
    function isValidComposition(
        uint256 humanCount,
        uint256 aiCount
    ) external pure returns (bool valid);
}
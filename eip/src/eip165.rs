//! [eip-165](https://eips.ethereum.org/EIPS/eip-165)

use anyhow::Result;

/// The interface identifier for this interface is 0x01ffc9a7. You can calculate this by running bytes4(keccak256('supportsInterface(bytes4)')); or using the Selector contract above.
///
/// Therefore the implementing contract will have a supportsInterface function that returns:
///
/// - true when interfaceID is 0x01ffc9a7 (EIP165 interface)
/// - false when interfaceID is 0xffffffff
/// - true for any other interfaceID this contract implements
/// - false for any other interfaceID
/// This function must return a bool and use at most 30,000 gas.
///
/// Implementation note, there are several logical ways to implement this function. Please see the example implementations and the discussion on gas usage.
pub trait ERC165 {
    /// @notice Query if a contract implements an interface
    /// @param interfaceID The interface identifier, as specified in ERC-165
    /// @dev Interface identification is specified in ERC-165. This function
    ///  uses less than 30,000 gas.
    /// @return `true` if the contract implements `interfaceID` and
    ///  `interfaceID` is not 0xffffffff, `false` otherwise
    fn supports_interface(&self, interface_id: &[u8; 4]) -> Result<bool>;
}

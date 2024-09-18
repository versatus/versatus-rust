use anyhow::Result;
use ethnum::U256;
use serde_derive::{Deserialize, Serialize};
use serde_hex::{SerHex, StrictPfx};
use std::io::{self, Read, Write};

/// This trait will generally be implemented by all Smart Contracts, as it gives us a way to make
/// all of the contract's inputs available to the contract itself.
pub trait SmartContract {
    /// A function to receive contract inputs. If the contract doesn't do anything with inputs,
    /// this can be empty. It can also do validation on the inputs and return an error if there is
    /// a problem with the input data provided. In general, smart contracts will likely want to
    /// stash this somewhere in &self for use in smart contract functions.
    fn receive_inputs(&mut self, inputs: &mut SmartContractInputs) -> Result<()>;
}

/// SmartContractInputs represents the entire bundle of inputs sent into a Versatus smart contract.
/// It is a collection of input data from a variety of locations, including the contract caller,
/// and the protocol accounts database.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SmartContractInputs {
    /// Version of the compute stack and API/ABI
    pub version: i32,
    /// Account info as provided by the protocol
    pub account_info: AccountInfo,
    /// Protocol inputs as provided by the protocol
    pub protocol_input: ProtocolInputs,
    /// Application inputs as provided by the application via the protocol
    pub contract_input: ContractInputs,
}

impl SmartContractInputs {
    /// Read JSON data on stdin and deserialise it to a set of Rust data structures.
    pub fn gather() -> Result<Self> {
        let mut json_data: Vec<u8> = vec![];
        let _num_bytes = io::stdin().read_to_end(&mut json_data)?;
        Ok(serde_json::from_slice(&json_data)?)
    }

    /// Returns a copy of the associated account information
    pub fn account_info(&self) -> AccountInfo {
        self.account_info.clone()
    }

    /// Returns the address of the associated account
    pub fn account_addr(&self) -> Address {
        self.account_info.account_address.clone()
    }
}

/// ContractInputs is a structure representing the inputs to a smart contract and generally equates
/// to what the Versatus protocol would receive on a public RPC request to execute a contract.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContractInputs {
    /// A string representing a function within the contract to call. This allows
    /// a single WASM binary to provide multiple functionalities. The idea would
    /// be that a main() function in the contract could switch between them based
    /// on a comparison with this value.
    pub contract_fn: String,
    /// Inputs passed from the caller to pass into the contract function.
    pub function_inputs: FunctionInputs,
}

/// ProtocolInputs represents inputs provided from the protocol that may be useful in smart
/// contracts.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolInputs {
    /// An internal version number for the protocol at this point in time
    pub version: i32,
    /// The block number/height of the block currently being processed
    pub block_height: u64,
    /// The timestamp of the block currently being processed
    pub block_time: u64,
}

/// AccountInfo represents the state of the account calling the smart contract and is provided by
/// the Versatus protocol.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    /// Address of the smart contract's blockchain account
    pub account_address: Address,
    /// Current balance of the smart contract's account at last block
    pub account_balance: U256,
}

/// FunctionInputs represents the data provided by the contract caller to be used as inputs into
/// the function being called within the smart contract.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum FunctionInputs {
    Erc20(crate::eip20::Erc20Inputs),
}

/// A high-level struct representing the output of a smart contract.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SmartContractOutputs {
    pub result: Vec<ContractResult>,
}

/// A smart contract result. Will generally equate to one of a number of known contract types, such
/// as ERC20 or ERC721.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ContractResult {
    Erc20(crate::eip20::Erc20Result),
    Erc721(()),
}

impl SmartContractOutputs {
    /// Writes smart contract output on stdout.
    pub fn commit(&self) -> Result<()> {
        Ok(io::stdout().write_all(serde_json::to_string(&self)?.as_bytes())?)
    }
}

/// A structure to represent an address (a slice of 20 bytes)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Address(#[serde(with = "SerHex::<StrictPfx>")] pub [u8; 20]);

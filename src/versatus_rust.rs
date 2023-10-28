use anyhow::{Error, Result};
use ethnum::U256;
use serde_derive::{Deserialize, Serialize};
use std::io::{self, Read, Write};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationInputs {
    /// A string representing a function within the contract to call. This allows
    /// a single WASM binary to provide multiple functionalities. The idea would
    /// be that a main() function in the contract could switch between them based
    /// on a comparison with this value.
    pub contract_fn: String,
    /// An amount to spend from contract's wallet
    pub amount: u64,
    /// A list of recipients (as a temporary substitute for proper user-defined inputs).
    pub recipients: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolInputs {
    /// An internal version number for the protocol at this point in time
    pub version: i32,
    /// The block number/height of the block currently being processed
    pub block_height: u64,
    /// The timestamp of the block currently being processed
    pub block_time: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    /// Address of the smart contract's blockchain account
    pub account_address: String,
    /// Current balance of the smart contract's account at last block
    pub account_balance: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComputeInputs {
    /// Version of the compute stack and API/ABI
    pub version: i32,
    /// Account info as provided by the protocol
    pub account_info: AccountInfo,
    /// Protocol inputs as provided by the protocol
    pub protocol_input: ProtocolInputs,
    /// Application inputs as provided by the application via the protocol
    pub application_input: ApplicationInputs,
}

impl ComputeInputs {
    pub fn gather() -> Result<Self> {
        let mut json_data: Vec<u8> = vec![];
        let _num_bytes = io::stdin().read_to_end(&mut json_data)?;
        Ok(serde_json::from_slice(&json_data)?)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComputeOutputs {
    pub transactions: Vec<ComputeTransaction>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComputeTransaction {
    pub recipient: String,
    pub amount: u64,
}

// XXX: TODO: turn this into a builder
impl ComputeOutputs {
    pub fn commit(&self) -> Result<()> {
        Ok(io::stdout().write_all(serde_json::to_string(&self)?.as_bytes())?)
    }
}

// ERC20 related structures and interfaces
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Address {
    pub bytes: [u8; 20],
}

/// An interface for ERC20 contracts to conform to.
pub trait Erc20 {
    /// Optional. Token name.
    fn name(&self) -> Result<String, Error>;
    /// Optional. Token ticker symbol.
    fn symbol(&self) -> Result<String, Error>;
    /// Optional. Returns the number of decimals the token uses - e.g. 8, means to divide the token amount by
    /// 100000000 to get its user representation.
    fn decimals(&self) -> Result<u8, Error>;
    /// Returns the total token supply.
    fn total_supply(&self) -> Result<U256, Error>;
    /// Returns the account balance of another account with address [owner].
    fn balance_of(&self, owner: Address) -> Result<U256, Error>;
    /// Transfers _value amount of tokens to address _to, and MUST fire the Transfer event.
    /// The function SHOULD throw if the message caller’s account balance does not have enough tokens to spend.
    fn transfer(&self, to: Address, value: U256) -> Result<bool, Error>;
    /// Transfers _value amount of tokens from address _from to address _to, and MUST fire the Transfer event.
    ///
    /// The transferFrom method is used for a withdraw workflow, allowing contracts to transfer tokens on
    /// your behalf. This can be used for example to allow a contract to transfer tokens on your behalf
    /// and/or to charge fees in sub-currencies. The function SHOULD throw unless the _from account has
    /// deliberately authorized the sender of the message via some mechanism.
    ///
    /// Note Transfers of 0 values MUST be treated as normal transfers and fire the Transfer event.
    fn transfer_from(&self, from: Address, to: Address, value: U256) -> Result<bool, Error>;
    /// Allows _spender to withdraw from your account multiple times, up to the _value amount. If this
    /// function is called again it overwrites the current allowance with _value.
    ///
    /// NOTE: To prevent attack vectors like the one described here and discussed here, clients SHOULD make
    /// sure to create user interfaces in such a way that they set the allowance first to 0 before setting
    /// it to another value for the same spender. THOUGH The contract itself shouldn’t enforce it, to
    /// allow backwards compatibility with contracts deployed before.
    fn approve(&self, spender: Address, value: U256) -> Result<bool, Error>;
    /// Returns the amount which _spender is still allowed to withdraw from _owner.
    fn allowance(&self, owner: Address, spender: Address) -> Result<U256, Error>;
}

pub struct Erc20TransferEvent {
    pub from: Address,
    pub to: Address,
    pub value: U256,
}

pub struct Erc20ApprovalEvent {
    pub owner: Address,
    pub spender: Address,
    pub value: U256,
}

pub fn process_erc20<T: Erc20>(t: &T) -> Result<bool, Error> {
    println!("{}: {}", t.symbol()?, t.name()?);
    println!("Supply: {}", t.total_supply()?);
    Ok(true)
}

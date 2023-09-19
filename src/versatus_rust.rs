use anyhow::Result;
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

use crate::versatus_rust::{
    Address, ContractResult, FunctionInputs, SmartContract, SmartContractInputs,
    SmartContractOutputs,
};
use anyhow::{anyhow, Result};
use ethnum::U256;
use serde_derive::{Deserialize, Serialize};

/// Erc20Inputs is an enum/union representing the possible ERC20 function inputs.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Erc20Inputs {
    Name(),
    Symbol(),
    Decimals(),
    TotalSupply(),
    BalanceOf {
        address: Address,
    },
    Transfer {
        address: Address,
        value: U256,
    },
    TransferFrom {
        from: Address,
        to: Address,
        value: U256,
    },
    Approve {
        address: Address,
        value: U256,
    },
    Allowance {
        owner: Address,
        spender: Address,
    },
}

/// Erc20Result is an enum/union representing the possible ERC20 function return values.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Erc20Result {
    Name(String),
    Symbol(String),
    Decimals(u8),
    TotalSupply(U256),
    BalanceOf(U256),
    Transfer(Erc20TransferEvent),
    TransferFrom(Erc20TransferEvent),
    Approve(Erc20ApprovalEvent),
    Allowance(U256),
}

/// An interface for ERC20 contracts to conform to.
pub trait Erc20 {
    /// Optional. Token name.
    fn name(&self) -> Result<String>;
    /// Optional. Token ticker symbol.
    fn symbol(&self) -> Result<String>;
    /// Optional. Returns the number of decimals the token uses - e.g. 8, means to divide the token amount by
    /// 100000000 to get its user representation.
    fn decimals(&self) -> Result<u8>;
    /// Returns the total token supply.
    fn total_supply(&self) -> Result<U256>;
    /// Returns the account balance of another account with address [owner].
    fn balance_of(&self, owner: Address) -> Result<U256>;
    /// Transfers _value amount of tokens to address _to, and MUST fire the Transfer event.
    /// The function SHOULD throw if the message caller’s account balance does not have enough tokens to spend.
    fn transfer(&self, to: Address, value: U256) -> Result<Erc20TransferEvent>;
    /// Transfers _value amount of tokens from address _from to address _to, and MUST fire the Transfer event.
    ///
    /// The transferFrom method is used for a withdraw workflow, allowing contracts to transfer tokens on
    /// your behalf. This can be used for example to allow a contract to transfer tokens on your behalf
    /// and/or to charge fees in sub-currencies. The function SHOULD throw unless the _from account has
    /// deliberately authorized the sender of the message via some mechanism.
    ///
    /// Note Transfers of 0 values MUST be treated as normal transfers and fire the Transfer event.
    fn transfer_from(&self, from: Address, to: Address, value: U256) -> Result<Erc20TransferEvent>;
    /// Allows _spender to withdraw from your account multiple times, up to the _value amount. If this
    /// function is called again it overwrites the current allowance with _value.
    ///
    /// NOTE: To prevent attack vectors like the one described here and discussed here, clients SHOULD make
    /// sure to create user interfaces in such a way that they set the allowance first to 0 before setting
    /// it to another value for the same spender. THOUGH The contract itself shouldn’t enforce it, to
    /// allow backwards compatibility with contracts deployed before.
    fn approve(&self, spender: Address, value: U256) -> Result<Erc20ApprovalEvent>;
    /// Returns the amount which _spender is still allowed to withdraw from _owner.
    fn allowance(&self, owner: Address, spender: Address) -> Result<U256>;
}

/// Erc20TransferEvent is a struct to represent an ERC20 Transfer Event
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Erc20TransferEvent {
    pub from: Address,
    pub to: Address,
    pub value: U256,
}

/// Erc20ApprovalEvent is a struct to represent an ERC20 Approval Event
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Erc20ApprovalEvent {
    pub owner: Address,
    pub spender: Address,
    pub value: U256,
}

pub fn process_erc20<T: Erc20 + SmartContract>(contract: &mut T) -> Result<()> {
    // Read and parse stdin
    let mut input = SmartContractInputs::gather()?;

    // If the caller has asked us to, save the inputs.
    contract.receive_inputs(&mut input)?;

    let result: Erc20Result;

    // Call correct function
    match input.contract_input.contract_fn.as_str() {
        // XXX: Need to use let match to retrieve input arguments....
        "allowance" => {
            let owner: Address;
            let spender: Address;
            result = {
                match input.contract_input.function_inputs {
                    FunctionInputs::Erc20(Erc20Inputs::Allowance {
                        owner: in_owner,
                        spender: in_spender,
                    }) => {
                        owner = in_owner;
                        spender = in_spender;
                    }
                    _ => return Err(anyhow!("Contract inputs don't match allowance function")),
                }
                Erc20Result::Allowance(contract.allowance(owner, spender)?)
            }
        }
        "approve" => {
            let spender: Address;
            let value: U256;
            result = {
                match input.contract_input.function_inputs {
                    FunctionInputs::Erc20(Erc20Inputs::Approve {
                        address: in_spender,
                        value: in_value,
                    }) => {
                        spender = in_spender;
                        value = in_value;
                    }
                    _ => return Err(anyhow!("Contract inputs don't match approve function")),
                }
                Erc20Result::Approve(contract.approve(spender, value)?)
            }
        }
        "balance_of" => {
            let addr: Address;
            result = {
                match input.contract_input.function_inputs {
                    FunctionInputs::Erc20(Erc20Inputs::BalanceOf { address: in_addr }) => {
                        addr = in_addr;
                    }
                    _ => return Err(anyhow!("Contract inputs don't match balance_of function")),
                }
                Erc20Result::BalanceOf(contract.balance_of(addr)?)
            }
        }
        "total_supply" => {
            result = Erc20Result::TotalSupply(contract.total_supply()?);
        }
        "transfer" => {
            let to: Address;
            let value: U256;
            result = {
                match input.contract_input.function_inputs {
                    FunctionInputs::Erc20(Erc20Inputs::Transfer {
                        address: in_to,
                        value: in_value,
                    }) => {
                        to = in_to;
                        value = in_value;
                    }
                    _ => return Err(anyhow!("Contract inputs don't match transfer function")),
                }
                Erc20Result::Transfer(contract.transfer(to, value)?)
            }
        }
        "transfer_from" => {
            let from: Address;
            let to: Address;
            let value: U256;
            result = {
                match input.contract_input.function_inputs {
                    FunctionInputs::Erc20(Erc20Inputs::TransferFrom {
                        from: in_from,
                        to: in_to,
                        value: in_value,
                    }) => {
                        from = in_from;
                        to = in_to;
                        value = in_value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Contract inputs don't match transfer_from function"
                        ))
                    }
                }
                Erc20Result::TransferFrom(contract.transfer_from(from, to, value)?)
            }
        }
        "name" => {
            result = Erc20Result::Name(contract.name()?);
        }
        "symbol" => {
            result = Erc20Result::Symbol(contract.symbol()?);
        }
        "decimals" => {
            result = Erc20Result::Decimals(contract.decimals()?);
        }
        _ => {
            return Err(anyhow!(
                "Invalid contract function: {}",
                &input.contract_input.contract_fn
            ))
        }
    }

    let output = SmartContractOutputs {
        result: vec![ContractResult::Erc20(result)],
    };

    output.commit()?;
    Ok(())
}

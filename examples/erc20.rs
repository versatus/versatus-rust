//! A sample stub for an ERC20 token. Doesn't actually do anything yet. But compiles, runs, and
//! shows output. It's the bare-minimum required for an ERC20 contract.
use anyhow::{anyhow, Result};
use ethnum::U256;
use versatus_rust::eip20::{process_erc20, Erc20, Erc20ApprovalEvent, Erc20TransferEvent};
use versatus_rust::versatus_rust::{Address, SmartContract, SmartContractInputs};

#[derive(Clone, Debug)]
struct ComputeUnitToken {
    inputs: Option<SmartContractInputs>,
}

const COMPUTE_NAME: &str = "Compute Unit";
const COMPUTE_SYMBOL: &str = "COUN";
const COMPUTE_DECIMALS: u8 = 6;

impl SmartContract for ComputeUnitToken {
    fn receive_inputs(&mut self, inputs: &mut SmartContractInputs) -> Result<()> {
        self.inputs = Some(inputs.to_owned());
        Ok(())
    }
}

impl Erc20 for ComputeUnitToken {
    fn name(&self) -> Result<String> {
        Ok(COMPUTE_NAME.to_string())
    }

    fn symbol(&self) -> Result<String> {
        Ok(COMPUTE_SYMBOL.to_string())
    }

    fn decimals(&self) -> Result<u8> {
        Ok(COMPUTE_DECIMALS)
    }

    fn total_supply(&self) -> Result<U256> {
        Ok(U256::MAX)
    }

    fn balance_of(&self, _owner: Address) -> Result<U256> {
        Ok(U256::MAX)
    }

    fn transfer(&self, to: Address, value: U256) -> Result<Erc20TransferEvent> {
        let addr: Address;
        if let Some(input) = &self.inputs {
            addr = input.account_addr();
        } else {
            // Shouldn't be reachable.
            return Err(anyhow!("Input data missing"));
        }
        Ok(Erc20TransferEvent {
            from: addr,
            to,
            value,
        })
    }

    fn transfer_from(&self, from: Address, to: Address, value: U256) -> Result<Erc20TransferEvent> {
        Ok(Erc20TransferEvent { from, to, value })
    }

    fn approve(&self, spender: Address, value: U256) -> Result<Erc20ApprovalEvent> {
        let owner: Address;
        if let Some(input) = &self.inputs {
            owner = input.account_addr();
        } else {
            // Shouldn't be reachable.
            return Err(anyhow!("Input data missing"));
        }
        Ok(Erc20ApprovalEvent {
            owner,
            spender,
            value,
        })
    }

    fn allowance(&self, _owner: Address, _spender: Address) -> Result<U256> {
        Ok(U256::MIN)
    }
}

fn main() {
    let mut token = ComputeUnitToken { inputs: None };
    process_erc20(&mut token).unwrap();
}

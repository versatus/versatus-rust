use anyhow::Error;
use ethnum::{uint, U256};
use versatus_rust::versatus_rust::{process_erc20, Address, Erc20};

struct ComputeUnitToken {}

const COMPUTE_NAME: &str = "Compute Unit";
const COMPUTE_SYMBOL: &str = "COUN";
const COMPUTE_DECIMALS: u8 = 6;
const COMPUTE_SUPPLY: U256 = uint!("0xffff_ffff_ffff_ffff");

impl Erc20 for ComputeUnitToken {
    fn name(&self) -> Result<String, Error> {
        Ok(COMPUTE_NAME.to_string())
    }

    fn symbol(&self) -> Result<String, Error> {
        Ok(COMPUTE_SYMBOL.to_string())
    }

    fn decimals(&self) -> Result<u8, Error> {
        Ok(COMPUTE_DECIMALS)
    }

    fn total_supply(&self) -> Result<U256, Error> {
        Ok(COMPUTE_SUPPLY)
    }

    fn balance_of(&self, _owner: Address) -> Result<U256, Error> {
        Ok(uint!("0"))
    }

    fn transfer(&self, _to: Address, _value: U256) -> Result<bool, Error> {
        Ok(true)
    }

    fn transfer_from(&self, _from: Address, _to: Address, _value: U256) -> Result<bool, Error> {
        Ok(true)
    }

    fn approve(&self, _spender: Address, _value: U256) -> Result<bool, Error> {
        Ok(true)
    }

    fn allowance(&self, _owner: Address, _spender: Address) -> Result<U256, Error> {
        Ok(uint!("0"))
    }
}

fn main() {
    let token = ComputeUnitToken {};
    process_erc20(&token).unwrap();
}

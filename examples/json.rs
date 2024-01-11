/// This sample isn't for the long-term. But rather as a short-term way to be able to quickly
/// generate some JSON from the Rust data structures to aid in developing other code. This will end
/// up being re-created elsewhere with more functionality.
use ethnum::U256;
use std::io::{self, Write};
use versatus_rust::eip20::{Erc20Inputs::Transfer, Erc20Result::Symbol};
use versatus_rust::versatus_rust::{
    AccountInfo, Address, ContractInputs, ContractResult, FunctionInputs, ProtocolInputs,
    SmartContractInputs, SmartContractOutputs,
};

fn main() {
    let sci = SmartContractInputs {
        version: 1,
        account_info: AccountInfo {
            account_address: Address([2; 20]),
            account_balance: U256::MAX,
        },
        protocol_input: ProtocolInputs {
            version: 1,
            block_height: 1,
            block_time: 1,
        },
        contract_input: ContractInputs {
            contract_fn: "name".to_string(),
            function_inputs: FunctionInputs::Erc20(Transfer {
                address: Address([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 1]),
                value: U256::MAX,
            }),
        },
    };

    let sco = SmartContractOutputs {
        result: vec![ContractResult::Erc20(Symbol("COUN".to_string()))],
    };

    let t = (sci, sco);

    io::stdout()
        .write_all(serde_json::to_string(&t).unwrap().as_bytes())
        .unwrap();
}

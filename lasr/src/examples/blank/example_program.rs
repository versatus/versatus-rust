use crate::lasrctl::builders::program::Program;
use anyhow::Ok;
use lasr_types::*;
use std::io::Read;

pub struct BlankProgram {}

impl BlankProgram {
    pub fn hello(inputs: Inputs) -> Result<String, anyhow::Error> {
        let blank = Program::new();
        let outputs = blank
            .execute_method(&inputs)
            .map_err(|e| anyhow::anyhow!("failed to update program: {e:?}"))?;

        Ok(outputs)
    }
}

#[allow(dead_code)]
/// A minimalistic main function for a Rust LASR program.
/// Takes in lasr_type::Inputs, handles the call based on the program method, and produces necessary lasr_types::Outputs to be processed by protocol
fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let compute_inputs: Inputs = serde_json::from_str(&input)?;
    let program = Program::new();
    let result = program
        .execute_method(&compute_inputs)
        .map_err(|e| e.to_string())
        .unwrap();

    let json_output = serde_json::to_string(&result)?;
    println!("{json_output}");

    Ok(())
}

use std::io::Read;

use crate::lasrctl::builders::program::Program;

#[allow(dead_code)]
#[tokio::main]
/// A minimalistic main function for a Rust LASR program.
/// Takes in lasr_type::Inputs, handles the call based on the program method, and produces necessary lasr_types::Outputs to be processed by protocol
async fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let compute_inputs: lasr_types::Inputs = serde_json::from_str(&input)?;
    let program = Program::new();
    let result = program
        .execute_method(&compute_inputs)
        .map_err(|e| e.to_string())
        .unwrap();

    let json_output = serde_json::to_string(&result)?;
    println!("{json_output}");

    Ok(())
}

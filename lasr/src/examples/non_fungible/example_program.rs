use crate::lasrctl::builders::program::Program;
use lasr_types::Inputs;
use std::fs;

#[allow(dead_code)]
/// A minimalistic main function for a Rust LASR program.
/// Takes in lasr_type::Inputs, handles the call based on the program method, and produces necessary lasr_types::Outputs to be processed by protocol
async fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("./example-program-inputs.json")?;

    let compute_inputs: Inputs = serde_json::from_str(&input)?;
    let program = Program::new();
    let result = program
        .start(&compute_inputs)
        .map_err(|e| e.to_string())
        .unwrap();

    let json_output = serde_json::to_string(&result)?;
    println!("{}", &json_output);

    Ok(())
}

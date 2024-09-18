//! A minimalistic main function for a Rust LASR program.
//! A Program that takes in lasr_type::Inputs, handles the call based on the program method, and produces necessary lasr_types::Outputs to be processed by protocol
use crate::lasrctl::builders::program::Program;
use lasr_types::Inputs;
use std::fs;

#[allow(dead_code)]
fn main() -> anyhow::Result<()> {
    if let Err(e) = Program::run() {
        bail!("Error occured while running program: {}", e.to_string())
    }
    Ok(())
}

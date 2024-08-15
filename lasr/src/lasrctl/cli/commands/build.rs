use std::{path::PathBuf, process::Stdio};

use clap::Args;
use lasr_types::Inputs;

use crate::lasrctl::builders::program::Program;

#[derive(Args, Debug)]
pub struct BuildArgs {
    /// Contract file to include in the build
    file: Option<PathBuf>,
    /// Build target. Options: "cargo" or "wasm"
    #[arg(default_value = "cargo")]
    target: String,
}

impl BuildArgs {
    pub fn lasr_build(&self) -> anyhow::Result<()> {
        let output = std::process::Command::new("cargo")
            .arg("build")
            .arg("--release")
            .stdout(Stdio::piped())
            .output()?;

        if output.status.success() {
            let stdout = output.stdout;
            serde_json::to_string(&stdout).map_err(|e| {
                anyhow::anyhow!("failed to serialize outputs from LASR program: {e:?}")
            })?;
            println!("Successfully built LASR Program, outputs ready for testing!");
            Ok(())
        } else {
            eprintln!("Failed to build LASR Program, check Inputs and try again...");
            Err(anyhow::anyhow!(
                "Error: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    // Program outputs get processed/created via the inputs being fed through the Program's method execution.
    pub fn build_outputs(inputs: Inputs) -> anyhow::Result<String> {
        let program = Program::new();
        let result = program
            .execute_method(&inputs)
            .map_err(|e| e.to_string())
            .unwrap();

        let json_output = serde_json::to_string(&result)?;
        println!("{json_output}");

        Ok(json_output)
    }
}

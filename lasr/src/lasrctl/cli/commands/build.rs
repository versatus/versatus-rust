use std::path::Path;

use clap::Args;
use lasr_types::Inputs;

use crate::lasrctl::builders::program::Program;

#[derive(Args, Debug)]
pub struct BuildArgs {
    /// Contract file to include in the build
    file: String,
    /// Build target. Options: "cargo" or "wasm"
    #[arg(default_value = "cargo")]
    target: String,
}

impl BuildArgs {
    fn _parse_path(path_str: &str) -> (String, String, String, String, String) {
        let path = Path::new(path_str);

        let root = path
            .components()
            .next()
            .unwrap()
            .as_os_str()
            .to_string_lossy()
            .to_string();
        let dir = path
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_string_lossy()
            .to_string();
        let base = path
            .file_name()
            .unwrap_or_else(|| "".as_ref())
            .to_string_lossy()
            .to_string();
        let ext = path
            .extension()
            .unwrap_or_else(|| "".as_ref())
            .to_string_lossy()
            .to_string();
        let name = path
            .file_stem()
            .unwrap_or_else(|| "".as_ref())
            .to_string_lossy()
            .to_string();

        (root, dir, base, ext, name)
    }

    pub fn cargo_build() -> anyhow::Result<String> {
        let output = std::process::Command::new("cargo")
            .arg("build")
            .arg("--release")
            .output()?;

        if output.status.success() {
            let stdout = output.stdout;
            let result = serde_json::to_string(&stdout).map_err(|e| {
                anyhow::anyhow!("failed to serialize outputs from LASR program: {e:?}")
            })?;
            println!("Successfully built LASR Program, outputs ready for testing!");
            Ok(result)
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

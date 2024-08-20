use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    process::Stdio,
};

use clap::Args;
use lasr_types::Inputs;

use crate::{lasrctl::builders::program::Program, scripts::consts::PROGRAM_OUTPUT_FILENAME};

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
    pub fn build_outputs_to_file(inputs: Inputs) -> anyhow::Result<()> {
        // Establish working dir
        let project_dir = &env::current_dir()?;
        if !project_dir.is_dir() {
            anyhow::bail!(format!("{project_dir:?} is not a valid directory."));
        }

        // Establish program output dir
        let mut output_dir = Path::new(&project_dir).join(".my-proj");
        fs::create_dir_all(&output_dir)?;
        output_dir.push(PROGRAM_OUTPUT_FILENAME);

        // Inputs are executed by program, and returns Outputs used with `lasr_cli`
        let program = Program::new();
        let result = program
            .execute_method(&inputs)
            .map_err(|e| e.to_string())
            .unwrap();

        if !output_dir.exists() {
            println!("Generating new Program outputs at {output_dir:?}");
            let json_output = serde_json::to_string(&result)?;
            let mut f = fs::File::options()
                .create(true)
                .write(true)
                .open(output_dir.clone())
                .unwrap();
            writeln!(&mut f, "{json_output}").unwrap();
        } else {
            println!("Found existing LASR Program Outputs! In order to avoid accidental overwrites, please remove the `temp.json` file and try again.")
        }

        Ok(())
    }
}

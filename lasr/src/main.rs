//! 1) Create New Rust Project
//! ```
//! cargo new your-project-name
//! cd your-project-name
//! ```
//! 2) Install the `lasr-rust` crate
//! ```
//! cargo add lasr-rust
//! cargo update
//! ```
//! 3) Initialize Project with `lasrctl`
//! ```
//! cargo run lasrctl init hello-lasr
//! ```
//! Copies the necessary files for building lasr programs including:
//!     - `keypair.json` file containing new wallet keys,
//!     - `example_program_inputs.json` file containing a sample of the JSON inputs used when building LASR programs,
//!     - `YOUR_PROJECT_NAME` new Rust Project with an example LASR Program as `main.rs`.
//!
//! 4) Build Your Program
//! ```
//! cargo run lasrctl build example-program.rs
//! ```
//! Runs example/user Inputs though main fn, and returns Outputs that can later be deployed as a Program via `lasr_cli`
//! TODO: Find way aquire inputs in variable for build execution
//! 5) Test Your Program
//! ```
//! cargo run lasrctl test --build example-program --input_json example-program-inputs
//! ```
//! Tests the given Outputs via `lasr_cli`, upon success the Program is ready for deployment
//! 6) Create Account and Deploy Program
//! ```
//! cargo run lasrctl deploy --build example-program --symbol MYTOKEN --program_name "My first token on LASR"
//! ```
//! Deploy a LASR Program via `lasr_cli
//!
//! TODO: Re-write the above to be module level documentation about the program, you may need to move certain
//! parts around for it to make sense. Write it as if someone were to have to read it as instructions of how to use
//! your program if they knew almost nothing about LASR.

use std::process::Stdio;

use clap::Parser;
use lasr_rust::lasrctl::cli::commands::build::BuildArgs;
use lasr_rust::lasrctl::cli::commands::init::InitArgs;
use lasr_rust::lasrctl::cli::LasrCommand;
use lasr_rust::lasrctl::cli::LasrCtl;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    match LasrCtl::parse().command() {
        LasrCommand::Init(init_args) => InitArgs::lasr_init(&init_args)
            .map_err(|e| anyhow::anyhow!("failed to initalize LASR program: {e:?}"))?,
        LasrCommand::Build(build_args) => BuildArgs::lasr_build(&build_args)
            .map_err(|e| anyhow::anyhow!("failed to build LASR program outputs: {e:?}"))?,
        LasrCommand::Test(test_args) => {
            let result = test_args.test_program();
            if let Ok(output) = &result {
                match output.status.code() {
                    Some(0) => {
                        let outputs_json = String::from_utf8_lossy(&output.stdout);
                        println!("successfully retreived program outputs: {}", &outputs_json);
                        let handle = std::process::Command::new("lasr_cli")
                            .args(&["parse-outputs", "-j", &outputs_json])
                            .spawn()
                            .map_err(|e| {
                                anyhow::anyhow!("failed to spawn child process for lasr_cli: {e:?}")
                            })?;
                        let program_outputs = handle.wait_with_output().map_err(|e| {
                            anyhow::anyhow!("failed to verify json outputs via lasr_cli: {e:?}")
                        })?;
                        println!("{}", String::from_utf8_lossy(&program_outputs.stdout));
                        eprintln!("{}", String::from_utf8_lossy(&program_outputs.stderr));
                    }
                    Some(code) => {
                        anyhow::bail!("encountered an error while attempting to retreive program outputs: {} {}", String::from_utf8_lossy(&output.stderr), code);
                    }
                    None => anyhow::bail!(
                        "encountered an error while attempting to retreive program outputs"
                    ),
                }
            }
        }

        LasrCommand::Deploy(_) => todo!(),
        LasrCommand::Call(_) => todo!(),
        LasrCommand::Send(_) => todo!(),
    }

    Ok(())
}

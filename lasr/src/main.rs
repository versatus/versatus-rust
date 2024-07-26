//! Explain the problem and how you plan to approach solving it.
//! Problem: No sdk for rust, so rust users would have to learn the innerworkings of lasr to try and use any CUTB methods
//! Solution: SDK that handles communitation to network via CUTB methods
//! 1) Using the `clap` crate create simple functions to parse inputs via CLI to create expected CUTB method
//!     -Inputs will vary from method types (fungible/non-fungible/blank)
//!     -Functions to handle Instruction building for relevant inputs -> Instructions
//!     -Inputs(ComputeInputs) sent with (Instructions) to lasr as json
//!     -Store inputs in a file path that gets used to send payloads
//!     -Add default input json file to test created input file structure
//!     -Add default wallet keypair file that gets used on program registration
//! 2) Send (ComputeInputs/Instructions) to lasr network for processing
//!     -Connect to lasr_node via RPC?
//!     -Supply lasr_node with instructions to process
//! 3) Recieve processed outputs from lasr network to relay to user on success or failure
//!     -Connect to lasr_node via RPC to listen for replies from network
//!
//! 1. Write your command line options!
//! 2. Test your command line with `cargo run --bin cli` for example.
//! 3. Build out the _intention_ of the project, not through copying the JS library.
//! #[derive(Command, Parser)]
//! struct LasrRustCommand { address: String, }
//! fn run(command: Command) -> Result<()> {
//!     let Command {
//!         address,
//!     } = command;
//!     do_something_with_addres(address)?;
//! }
//!
//! $ rlasr --address 0xSOMETHING
//! $ { address: "0xSOMETHING" }
//!
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
//! Copies the necessary files for building lasr programs
//!
//! 4) Build Your Program
//! ```
//! cargo run lasrctl build example-program.rs
//! ```
//! 5) Test Your Program
//! ```
//! cargo run lasrctl test --build example-program --input_json example-program-inputs
//! ```
//! 6) Create Account and Deploy Program
//! ```
//! cargo run lasrctl deploy --build example-program --symbol MYTOKEN --program_name "My first token on LASR"
//! ```
//!
//! TODO: Re-write the above to be module level documentation about the program, you may need to move certain
//! parts around for it to make sense. Write it as if someone were to have to read it as instructions of how to use
//! your program if they knew almost nothing about LASR.

use clap::Parser;
use lasr_rust::lasrctl::cli::LasrCommand;
use lasr_rust::{
    lasrctl::{
        cli::{InitArgs, LasrCtl},
        network::client::NetworkClient,
    },
    scripts::consts::{LASR_RPC_URL_STABLE, VIPFS_URL},
};

use anyhow::Ok;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let lasr_url = LASR_RPC_URL_STABLE;
    let ipfs_url = VIPFS_URL;
    let lasr_network = NetworkClient::new(lasr_url.to_string());
    let ipfs_network = NetworkClient::new(ipfs_url.to_string());

    match LasrCtl::parse().command() {
        LasrCommand::Init(init_args) => InitArgs::lasr_init(init_args)
            .await
            .map_err(|e| anyhow::anyhow!("failed to initalize LASR program: {e:?}"))?,
        LasrCommand::Build(_) => todo!(),
        LasrCommand::Test(_) => todo!(),
        LasrCommand::Deploy(_) => todo!(),
        LasrCommand::Call(_) => todo!(),
        LasrCommand::Send(_) => todo!(),
    }

    Ok(())
}

// cargo install lasr-rust
// 1. download the cli binary
//
// make a new directory where your lasr program will live
// 2. lasr-rust init blank
//
// Filetree example:
// Cargo.toml
// src
// |
// ---main.rs (should contain the blank program)

// deserialize the json file for blank into the rust types in lasr_types
// write them to the designated file, name it what it needs to be named
// likely since it's a rust program you'll need to use cargo to create a new project
// you can do this using std::process::Command which allows you to use the user's system
// to call programs the user has access to. If you didn't want to have to do it manually
// that way you could also just write some shell script and call that using std::process::Command (both are acceptable, neither is better than the other).
// would look kinda like:
// let blank_template = serde_json::deserialize("blank-create.json").unwrap(); // this isn't correct I forgot what the command is
// std::process::Command::new("cargo").arg("new").arg("--bin").arg("--path <path_to_folder>").output().unwrap();
// std::process::Command::new("cargo").arg("add").arg("lasr_types").output().unwrap();

#[macro_export]
macro_rules! cargo_command {
    ($($arg:expr),*) => {{
        let mut command = std::process::Command::new("cargo");
        $(command.arg($arg);)*
        match command.output() {
            std::result::Result::Ok(o) => Ok(o),
            Err(e) => {
                use std::fmt::Write;
                let args = [$($arg,)*];
                let mut args_str = String::with_capacity(args.len());
                args.iter()
                    .for_each(|arg| write!(args_str, "{arg} ").expect("failed to write cargo args into string buffer."));
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "cargo command failed: cargo {}\nError: {e:?}",
                        args_str
                    ),
                ))
            }
        }
    }};
}

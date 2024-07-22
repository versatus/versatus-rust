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
use lasr_rust::examples::blank::example_program::{init_program, Batman};
use lasr_rust::lasrctl::builders::program::{MethodStrategy, Program};
use lasr_rust::{
    lasrctl::{
        cli::{Command, InitArgs, LasrCtl},
        network::client::NetworkClient,
    },
    scripts::consts::{LASR_RPC_URL_STABLE, VIPFS_URL},
};
use std::io::Read;
use std::result;

use anyhow::Ok;
use lasr_types::*;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let compute_inputs: Inputs = serde_json::from_str(&input)?;
    let result = Batman::hello(compute_inputs)?;
    // let result = program
    //     .execute_method(&compute_inputs)
    //     .map_err(|e| e.to_string())
    //     .unwrap();

    let json_output = serde_json::to_string(&result)?;
    println!("{json_output}");

    Ok(())
}

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     let lasr_url = LASR_RPC_URL_STABLE;
//     let ipfs_url = VIPFS_URL;
//     let lasr_network = NetworkClient::new(lasr_url.to_string());
//     let ipfs_network = NetworkClient::new(ipfs_url.to_string());

//     match LasrCtl::parse().command() {
//         Command::Init(init_args) => {
//             println!("{init_args:?}");
//             lasr_init(init_args).await
//         } // lasr_init(init_args)?,
//         _ => todo!("this command is not added yet..."), // lasr_XXXX(XXXX_args)?,
//                                                         // etc,
//     }
// }

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
// pub async fn lasr_init(init_args: &InitArgs) -> anyhow::Result<()> {
//     let InitArgs {
//         blank,
//         fungible,
//         non_fungible,
//         faucet,
//     } = init_args;

//     if *blank {
//         let method = MethodStrategy::Create;
//         let template_str =
//             include_str!("./examples/blank/example-program-inputs/blank-create.json");
//         // probably what you want to do is deserialize the json into the lasr types first, then format them into a String.
//         // After you have the string you can just call the function I made below: init_template(template)?;
//         // See my comments in that function..
//         let map: Inputs = serde_json::from_str(&template_str)
//             .map_err(|e| anyhow::anyhow!("failed to destructure json template: {e:?}"))?;

//         // let transaction = &map.transaction;
//         // let txn_type = &map.transaction.transaction_type();
//         // let acct_info = &map.account_info;

//         // let acct_type = &acct_info.account_type();
//         // let programs = acct_info.programs();
//         // let program_acct_data = acct_info.program_account_data();
//         // let program_acct_meta = acct_info.program_account_metadata();

//         // A JSON object representative of a LASR Program
//         let program = init_program(method, map);
//     } else if *fungible {
//     } else if *non_fungible {
//     } else if *faucet {
//     } else {
//         return Ok(());
//     }
//     // deserialize the json file for blank into the rust types in lasr_types
//     // write them to the designated file, name it what it needs to be named
//     // likely since it's a rust program you'll need to use cargo to create a new project
//     // you can do this using std::process::Command which allows you to use the user's system
//     // to call programs the user has access to. If you didn't want to have to do it manually
//     // that way you could also just write some shell script and call that using std::process::Command (both are acceptable, neither is better than the other).
//     // would look kinda like:
//     // let blank_template = serde_json::deserialize("blank-create.json").unwrap(); // this isn't correct I forgot what the command is
//     // std::process::Command::new("cargo").arg("new").arg("--bin").arg("--path <path_to_folder>").output().unwrap();
//     // std::process::Command::new("cargo").arg("add").arg("lasr_types").output().unwrap();

//     Ok(())
// }
// fn init_template(_template: String) -> anyhow::Result<()> {
//     // this does not yet include writing the template code into a rust module
//     // or including the module in a module tree.
//     // it may be sufficient to overwrite the main.rs file with it
//     cargo_command!("init")?;
//     // This still needs to be added to the cargo registry... (crates.io)
//     // 'cargo add lasr_types' fails
//     cargo_command!("add", "--git", "https://github.com/versatus/lasr.git")?;
//     Ok(())
// }
// #[macro_export]
// macro_rules! cargo_command {
//     ($($arg:expr),*) => {{
//         let mut command = std::process::Command::new("cargo");
//         $(command.arg($arg);)*
//         match command.output() {
//             Ok(o) => Ok(o),
//             Err(e) => {
//                 use std::fmt::Write;
//                 let args = [$($arg,)*];
//                 let mut args_str = String::with_capacity(args.len());
//                 args.iter()
//                     .for_each(|arg| write!(args_str, "{arg} ").expect("failed to write cargo args into string buffer."));
//                 Err(std::io::Error::new(
//                     std::io::ErrorKind::Other,
//                     format!(
//                         "cargo command failed: cargo {}\nError: {e:?}",
//                         args_str
//                     ),
//                 ))
//             }
//         }
//     }};
// }

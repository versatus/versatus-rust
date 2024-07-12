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
use lasr_rust::{
    lasrctl::{
        cli::{Command, LasrCtl},
        network::client::NetworkClient,
    },
    scripts::consts::{LASR_RPC_URL_STABLE, VIPFS_URL},
};

fn main() {
    let lasr_url = LASR_RPC_URL_STABLE;
    let ipfs_url = VIPFS_URL;
    let lasr_network = NetworkClient::new(lasr_url.to_string());
    let ipfs_network = NetworkClient::new(ipfs_url.to_string());

    match LasrCtl::parse().command() {
        Command::Init(init_args) => {
            println!("{init_args:?}");
        } // lasr_init(init_args)?,
        _ => todo!("this command is not added yet..."), // lasr_XXXX(XXXX_args)?,
                                                        // etc,
    }
}

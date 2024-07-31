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
use lasr_rust::lasrctl::cli::commands::init::InitArgs;
use lasr_rust::lasrctl::cli::LasrCommand;
use lasr_rust::lasrctl::cli::LasrCtl;

use anyhow::Ok;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    match LasrCtl::parse().command() {
        LasrCommand::Init(init_args) => InitArgs::lasr_init(init_args)
            .map_err(|e| anyhow::anyhow!("failed to initalize LASR program: {e:?}"))?,
        LasrCommand::Build(_) => todo!(),
        LasrCommand::Test(_) => todo!(),
        LasrCommand::Deploy(_) => todo!(),
        LasrCommand::Call(_) => todo!(),
        LasrCommand::Send(_) => todo!(),
    }

    Ok(())
}

// #[tokio::main]
// /// A minimalistic main function for a Rust LASR program.
// /// Takes in lasr_type::Inputs, handles the call based on the program method, and produces necessary lasr_types::Outputs to be processed by protocol
// async fn main() -> anyhow::Result<()> {
//     let mut input = String::new();
//     std::io::stdin().read_to_string(&mut input)?;

//     let compute_inputs: lasr_types::Inputs = serde_json::from_str(&input)?;
//     let program = Program::new();
//     let result = program
//         .execute_method(&compute_inputs)
//         .map_err(|e| e.to_string())
//         .unwrap();

//     let json_output = serde_json::to_string(&result)?;
//     println!("{json_output}");

//     Ok(())
// }

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

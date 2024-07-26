use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail};
use clap::{ArgAction, ArgMatches, Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "lasr-rust", version = "1.0", about = "LASR Rust SDK")]
pub struct LasrCtl {
    #[clap(subcommand)]
    command: LasrCommand,
}
impl LasrCtl {
    pub fn command(&self) -> &LasrCommand {
        &self.command
    }
}

#[derive(Subcommand, Debug)]
pub enum LasrCommand {
    /// Initialize a project with an example program
    Init(InitArgs),
    /// Build the project with the specified contract
    Build(BuildArgs),
    /// Run the test suite for the project
    Test(TestArgs),
    /// Deploy a program to LASR
    Deploy(DeployArgs),
    /// Call a program method with the specified arguments
    Call(CallArgs),
    /// Send a specified amount of tokens to a recipient
    Send(SendArgs),
}

#[derive(Debug, Args)]
pub struct InitArgs {
    /// A minimal template to start from scratch
    #[arg(required = false, action = ArgAction::SetFalse)]
    pub blank: bool,
    /// A template for creating fungible tokens
    #[arg(required = false, action = ArgAction::SetFalse)]
    pub fungible: bool,
    /// A template for creating non-fungible tokens
    #[arg(required = false, action = ArgAction::SetFalse)]
    pub non_fungible: bool,
    /// A template for creating a faucet, allowing users to request test tokens
    #[arg(required = false, action = ArgAction::SetFalse)]
    pub faucet: bool,
}
impl InitArgs {
    fn init_template(
        project_dir: &PathBuf,
        json_content: &str,
        example_program: &str,
    ) -> anyhow::Result<()> {
        fs::create_dir_all(&project_dir)?;

        let src_dir = Path::new(&project_dir).join("src");
        fs::create_dir_all(&src_dir)?;

        let json_file_path = Path::new(&project_dir).join("example-program-inputs.json");
        let mut json_file = fs::File::create(json_file_path)?;
        json_file.write_all(json_content.as_bytes())?;

        let main_rs_path = src_dir.join("main.rs");
        let mut main_rs_file = fs::File::create(main_rs_path)?;
        main_rs_file.write_all(example_program.as_bytes())?;

        // Run `cargo init` to initialize the project as a cargo project
        let output = std::process::Command::new("cargo")
            .arg("init")
            .arg("--bin")
            .arg(project_dir)
            .output()?;
        if output.status.success() {
            println!("Successfully initalized LASR application folder");
        } else {
            eprintln!("Failed to initalized LASR application folder");
            eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
        }

        Ok(())
    }

    pub async fn lasr_init(init_args: &InitArgs) -> anyhow::Result<()> {
        match init_args {
            InitArgs { blank: true, .. } => {
                let project_dir = &env::current_dir()?;

                if !project_dir.is_dir() {
                    bail!(format!("{project_dir:?} is not a valid directory."));
                }

                dbg!(
                    "\nUsing project directory at {}",
                    project_dir.canonicalize()?.display()
                );

                let json_content =
                    include_str!("../examples/blank/example-program-inputs/blank-create.json");

                let example_program = include_str!("../examples/blank/example_program.rs");

                if let Err(e) = Self::init_template(&project_dir, &json_content, &example_program) {
                    eprintln!("Error initializing LASR program: {e:?}");
                    Ok(())
                } else {
                    println!("Initialization completed successfully!");
                    Ok(())
                }
            }
            InitArgs { fungible: true, .. } => todo!(),
            InitArgs {
                non_fungible: true, ..
            } => todo!(),
            InitArgs { faucet: true, .. } => todo!(),
            _ => Err(anyhow!("unsupported initialization method")),
        }
    }
}

#[derive(Args, Debug)]
pub struct BuildArgs {
    /// Contract file to include in the build
    file: String,
    /// Build target. Options: "cargo" or "wasm"
    #[arg(default_value = "cargo")]
    target: String,
}

#[derive(Args, Debug)]
pub struct TestArgs {
    /// Filename of the built program to be deployed. Ex: "example-program"
    #[arg(short = 'b')]
    build: String,
    /// Path to the JSON input file or dir containing JSON files for testing
    #[arg(short = 'i')]
    input_json: String,
}

#[derive(Args, Debug)]
pub struct DeployArgs {
    /// Filename of the built program to be deployed. Ex: "example-program"
    #[arg(short = 'b')]
    build: String,
    /// Author of the program
    #[arg(short = 'a')]
    author: String,
    /// Name of the program
    #[arg(short = 'n')]
    name: String,
    /// Symbol for the program
    #[arg(short = 's')]
    symbol: String,
    /// Name for the program
    #[arg(short = 'p')]
    program_name: String,
    /// Supply of the token to be sent to either the caller or the program
    #[arg(default_value = "1")]
    init_supply: String,
    /// Total supply of the token to be created
    #[arg(short = 't', default_value = "1")]
    total_supply: String,
    /// Address for the initialized supply
    #[arg(short = 'r')]
    recipient_address: String,
    /// Additional inputs for the program
    #[arg(default_value = "{}")]
    tx_inputs: String,
    /// Path to the keypair file
    #[arg(default_value = "./.lasr/wallet/keypair.json")]
    keypair_path: String,
    /// Secret key for the wallet
    #[arg(short = 'k')]
    secret_key: String,
    /// Network handle. Options: "stable" or "unstable"
    #[arg(short = 'x', default_value = "stable")]
    network: String,
}

#[derive(Args, Debug)]
pub struct CallArgs {
    /// Program address to be sent
    #[arg(short = 'p')]
    program_address: String,
    /// Operation to be preformed by the program
    op: String,
    /// Input json required by the operation
    tx_inputs: String,
    /// Value (in verse) to be sent to the program method
    value: String,
    /// Desired network. Options: "stable" or "test"
    #[arg(short = 'x', default_value = "stable")]
    network: String,
    /// Path to the keypair file
    #[arg(default_value = "./.lasr/wallet/keypair.json")]
    keypair_path: String,
    /// Secret key for the wallet
    #[arg(short = 'k')]
    secret_key: String,
}

#[derive(Args, Debug)]
pub struct SendArgs {
    /// Program address to be sent
    #[arg(short = 'p')]
    program_address: String,
    /// Amount to be sent (in Verse)
    amount: String,
    /// Address for the initialized supply
    #[arg(short = 'r')]
    recipient_address: String,
    /// Network to send on
    #[arg(short = 'x', default_value = "stable")]
    network: String,
    /// Path to the keypair file
    #[arg(default_value = "./.lasr/wallet/keypair.json")]
    keypair_path: String,
    /// Secret key for the wallet
    #[arg(short = 'k')]
    secret_key: String,
}

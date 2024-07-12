use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "lasr-rust", version = "1.0", about = "LASR Rust SDK")]
pub struct LasrCtl {
    #[clap(subcommand)]
    command: Command,
}
impl LasrCtl {
    pub fn command(&self) -> &Command {
        &self.command
    }
}

#[derive(Subcommand, Debug)]
pub enum Command {
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

#[derive(Args, Debug)]
pub struct InitArgs {
    /// A minimal template to start from scratch
    #[arg(default_value = "false")]
    blank: bool,
    /// A template for creating fungible tokens
    #[arg(default_value = "false")]
    fungible: bool,
    /// A template for creating non-fungible tokens
    #[arg(default_value = "false")]
    non_fungible: bool,
    /// A template for creating a faucet, allowing users to request test tokens
    #[arg(default_value = "false")]
    faucet: bool,
}
impl InitArgs {
    pub fn blank(&self) -> Option<&str> {
        const BLANK_TEMPLATE: &str = "";
        if self.blank {
            return Some(BLANK_TEMPLATE);
        }
        None
    }
    pub fn template(&self) -> anyhow::Result<Option<&str>> {
        match (self.blank, self.fungible, self.non_fungible, self.faucet) {
            (true, false, false, false) => { /* blank */},
            _ => anyhow::bail!("more than one template option selected, please pass only a single template option to continue."),
        }
        Ok(None)
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

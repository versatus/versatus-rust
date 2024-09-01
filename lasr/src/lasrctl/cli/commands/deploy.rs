use std::path::PathBuf;

use clap::Args;
use lasr_types::Inputs;

#[derive(Args, Debug)]
pub struct DeployArgs {
    /// Filename of the built program to be deployed. Ex: "path/to/example-program"
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

impl DeployArgs {
    fn handle_deploy(args: DeployArgs) -> anyhow::Result<()> {
        Ok(())
    }

    fn get_txn_inputs(inputs: Inputs) -> anyhow::Result<String> {
        let txn = inputs.transaction;
        let txn_inputs = txn.inputs();

        Ok(txn_inputs)
    }
}

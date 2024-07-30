use clap::Args;

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

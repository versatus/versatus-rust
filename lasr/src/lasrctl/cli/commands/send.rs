use clap::Args;

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

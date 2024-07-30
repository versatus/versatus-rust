use clap::Args;

#[derive(Args, Debug)]
pub struct BuildArgs {
    /// Contract file to include in the build
    file: String,
    /// Build target. Options: "cargo" or "wasm"
    #[arg(default_value = "cargo")]
    target: String,
}

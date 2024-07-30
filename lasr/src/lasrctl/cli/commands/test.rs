use clap::Args;

#[derive(Args, Debug)]
pub struct TestArgs {
    /// Filename of the built program to be deployed. Ex: "example-program"
    #[arg(short = 'b')]
    build: String,
    /// Path to the JSON input file or dir containing JSON files for testing
    #[arg(short = 'i')]
    input_json: String,
}

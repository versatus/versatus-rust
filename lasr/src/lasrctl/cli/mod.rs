pub mod commands;
use self::commands::{
    build::BuildArgs, call::CallArgs, deploy::DeployArgs, init::InitArgs, send::SendArgs,
    test::TestArgs,
};
use clap::{Parser, Subcommand};

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

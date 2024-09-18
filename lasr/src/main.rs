use std::process;

use clap::Parser;
use lasr_rust::lasrctl::cli::commands::build::BuildArgs;
use lasr_rust::lasrctl::cli::commands::init::InitArgs;
use lasr_rust::lasrctl::cli::LasrCommand;
use lasr_rust::lasrctl::cli::LasrCtl;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    match LasrCtl::parse().command() {
        LasrCommand::Init(init_args) => InitArgs::lasr_init(&init_args)
            .map_err(|e| anyhow::anyhow!("failed to initalize LASR program: {e:?}"))?,
        LasrCommand::Build(build_args) => BuildArgs::lasr_build(&build_args)
            .map_err(|e| anyhow::anyhow!("failed to build LASR program outputs: {e:?}"))?,
        LasrCommand::Test(test_args) => test_args.test_program().and_then(|output| {
            if output.status.success() {
                let outputs_json = String::from_utf8_lossy(&output.stdout);
                println!("successfully retreived program outputs: {}", &outputs_json);
                let handle = process::Command::new("lasr_cli")
                    .args(&["parse-outputs", "-j", &outputs_json])
                    .stdout(process::Stdio::piped())
                    .stderr(process::Stdio::piped())
                    .spawn()
                    .map_err(|e| {
                        anyhow::anyhow!("failed to spawn child process for lasr_cli: {e:?}")
                    })?;
                let program_outputs = handle.wait_with_output().map_err(|e| {
                    anyhow::anyhow!("failed to verify json outputs via lasr_cli: {e:?}")
                })?;

                let stdout = String::from_utf8_lossy(&program_outputs.stdout);
                let stderr = String::from_utf8_lossy(&program_outputs.stderr);
                if !stdout.is_empty() {
                    println!("lasr_cli stdout: {}", &stdout);
                }
                if !stderr.is_empty() {
                    eprintln!("lasr_cli stderr: {}", &stderr);
                }
                Ok(())
            } else {
                let mut stderr = 
                    String::from_utf8(output.stderr)?;
                if stderr.is_empty() {
                    stderr = String::from("process did not specify stderr");
                }
                match output.status.code() {
                    Some(code) => {
                        anyhow::bail!(
                    "encountered an error while attempting to retreive program outputs: {} code: {}",
                    stderr,
                    code
                );
                    }
                    None => {
                        anyhow::bail!(
                            "encountered an error while attempting to retreive program outputs"
                        )
                    }
                }
            }
        })?,

        LasrCommand::Deploy(_) => todo!(),
        LasrCommand::Call(_) => todo!(),
        LasrCommand::Send(_) => todo!(),
    }

    Ok(())
}

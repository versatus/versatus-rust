use clap::Args;
use std::{
    env,
    io::{self, Read},
    process::Stdio,
};

#[derive(Args, Debug)]
pub struct TestArgs {
    /// Filename of the built program to be deployed. Ex: "example-program"
    #[arg(short = 'b')]
    build: String,
    /// Path to the JSON input file or dir containing JSON files for testing
    #[arg(short = 'i')]
    input_json: String,
}

impl TestArgs {
    pub fn test_program(&self) -> anyhow::Result<()> {
        if env::current_dir().is_ok() {
            let build_path = std::path::PathBuf::from(&self.build);
            let input_path = std::path::PathBuf::from(&self.input_json);

            if build_path == env::current_exe()? {
                let json_input_str = &std::fs::read_to_string(input_path)?;
                io::stdin().read_to_string(&mut json_input_str.to_string())?;

                let run = std::process::Command::new(build_path)
                    .stdin(Stdio::piped())
                    .spawn()
                    .expect("Failed to run LASR Program.");
                let run_output = run.wait_with_output().unwrap();

                match run_output.status.code() {
                    Some(0) => println!("OK: {}", String::from_utf8_lossy(&run_output.stdout)),
                    Some(code) => println!("Error: {}", code),
                    None => {}
                }
            }
        }
        Ok(())
    }
}

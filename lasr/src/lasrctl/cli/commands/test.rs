use anyhow::bail;
use clap::Args;
use std::{
    io::Write,
    path::PathBuf,
    process::{Output, Stdio},
};

#[derive(Args, Debug)]
pub struct TestArgs {
    /// Filename of the built program to be deployed. Ex: "path/to/example-program"
    #[arg(short = 'b')]
    build: PathBuf,
    /// Path to the JSON input file or dir containing JSON files for testing
    #[arg(short = 'i')]
    input_json: PathBuf,
}

impl TestArgs {
    #[cfg(test)]
    pub fn new(build: PathBuf, input_json: PathBuf) -> Self {
        Self { build, input_json }
    }
    /// Takes a build path to a lasr program binary, and a path to some json inputs
    /// and feeds the json inputs as bytes to the binary program via stdin, returning
    /// the std::io::Output on success. This output can then be used more granularly
    /// for testing, debugging and printing useful information to a user.
    pub fn test_program(&self) -> anyhow::Result<Output> {
        if !self.build.exists() {
            bail!(
                "{:?} does not exist, please provide a valid path and try again.",
                self.build
            );
        }
        if !self.input_json.exists() {
            bail!(
                "{:?} does not exist, please provide a valid path and try again.",
                self.input_json
            );
        }

        println!("Searching for program path: {:?}", self.build);
        let json_input_str = &std::fs::read_to_string(&self.input_json).map_err(|e| {
            anyhow::anyhow!(
                "failed to read json inputs to string from path {:?}: {e:?}",
                self.input_json
            )
        })?;

        println!("Inputs discovered: {json_input_str:?}");
        let mut handle = std::process::Command::new(&self.build)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| {
                anyhow::anyhow!("failed to spawn child process for {:?}: {e:?}", self.build)
            })?;
        if let Some(mut stdin) = handle.stdin.take() {
            // Write the json_input to the child's stdin
            stdin
                .write_all(json_input_str.as_bytes())
                .map_err(|e| anyhow::anyhow!("failed to write json inputs to stdin: {e:?}"))?;
        } else {
            bail!("failed to acquire stdin for child process {:?}", self.build)
        }

        println!("Cargo project running...");
        handle
            .wait_with_output()
            .map_err(|e| anyhow::anyhow!("failed to run program at path: {:?}: {e:?}", self.build))
    }
}

#[cfg(test)]
mod test_args_tests {
    use super::TestArgs;
    #[test]
    fn test_program_works() {
        let cargo_manifest_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut build_path = cargo_manifest_path.clone();
        build_path.push("test_data/target/debug/test_data");
        let mut inputs_json = cargo_manifest_path;
        inputs_json.push("test_data/example-program-inputs.json");
        let args = TestArgs::new(build_path, inputs_json);
        let result = args.test_program();
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.status.code(), Some(0));
        assert_eq!(String::from_utf8_lossy(&output.stdout), String::from("{\n  \"computeInputs\": {\n    \"version\": 1,\n    \"accountInfo\": {\n      \"accountType\": {\n        \"program\": \"0x57234c52617e7ca8edc5577ebe3eb38d53a77607\"\n      },\n      \"programNamespace\": null,\n      \"ownerAddress\": \"0x482830d7655fb8465a43844fc1530a7713781b49\",\n      \"programs\": {},\n      \"nonce\": \"0x000000000000000000000000000000000000000000000000000000000000001c\",\n      \"programAccountData\": {},\n      \"programAccountMetadata\": {\n        \"content_id\": \"bafyreidhfvw4jiqom72332brsln3micsa4b7grur4rixkwvyrh6u4i3ecy\",\n        \"initializedSupply\": \"1000000000000000000000000\",\n        \"name\": \"Anotha One\",\n        \"symbol\": \"LOVE\",\n        \"to\": \"0x57234c52617e7ca8edc5577ebe3eb38d53a77607\",\n        \"totalSupply\": \"1000000000000000000000000\"\n      },\n      \"programAccountLinkedPrograms\": []\n    },\n    \"transaction\": {\n      \"transactionType\": {\n        \"call\": \"0x0000000000000000000000000000000000000000000000000000000000000001\"\n      },\n      \"from\": \"0x100444c7d04a842d19bc3ee63cb7b96682ff3f43\",\n      \"to\": \"0x100444c7d04a842d19bc3ee63cb7b96682ff3f43\",\n      \"programId\": \"0x100444c7d04a842d19bc3ee63cb7b96682ff3f43\",\n      \"op\": \"create\",\n      \"transactionInputs\": \"{\\\"name\\\":\\\"HelloToken\\\",\\\"symbol\\\":\\\"HLLO\\\",\\\"totalSupply\\\":\\\"1000\\\",\\\"initializedSupply\\\":\\\"1000\\\",\\\"imgUrl\\\":\\\"https://pbs.twimg.com/profile_images/1765199894539583488/RUiZn7jT_400x400.jpg\\\",\\\"paymentProgramAddress\\\":\\\"0x0000000000000000000000000000000000000000\\\",\\\"price\\\":\\\"1\\\",\\\"collection\\\":\\\"test\\\"}\",\n      \"value\": \"0x0000000000000000000000000000000000000000000000000000000000000001\",\n      \"nonce\": \"0x0000000000000000000000000000000000000000000000000000000000000001\",\n      \"v\": 1,\n      \"r\": \"0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef\",\n      \"s\": \"0xfedcba0987654321fedcba0987654321fedcba0987654321fedcba0987654321\"\n    },\n    \"op\": \"create\",\n    \"contractInputs\": \"{}\"\n  },\n  \"instructions\": [\n    {\n      \"create\": {\n        \"programNamespace\": \"this\",\n        \"programId\": \"this\",\n        \"programOwner\": \"0x100444c7d04a842d19bc3ee63cb7b96682ff3f43\",\n        \"totalSupply\": \"0x0000000000000000000000000000000000000000000000000000000000001000\",\n        \"initializedSupply\": \"0x0000000000000000000000000000000000000000000000000000000000001000\",\n        \"distribution\": []\n      }\n    },\n    {\n      \"update\": {\n        \"updates\": [\n          {\n            \"programUpdate\": {\n              \"account\": \"this\",\n              \"updates\": [\n                {\n                  \"field\": \"metadata\",\n                  \"value\": {\n                    \"metadata\": {\n                      \"insert\": [\n                        \"metadata\",\n                        \"[\\\"HLLO\\\",\\\"HelloToken\\\",\\\"1000\\\"]\"\n                      ]\n                    }\n                  }\n                }\n              ]\n            }\n          }\n        ]\n      }\n    }\n  ]\n}\n"));
    }
}

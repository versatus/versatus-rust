use crate::{cargo, scripts::consts::KEYPAIR_FILENAME};
use clap::Parser;
use jsonrpsee::http_client::HttpClient;
use lasr_wallet::Wallet;
use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug, Parser)]
#[clap(bin_name = "lasr init")]
pub struct InitArgs {
    /// A minimal template to start from scratch
    #[clap(long)]
    pub blank: bool,
    /// A template for creating fungible tokens
    #[clap(long)]
    pub fungible: bool,
    /// A template for creating non-fungible tokens
    #[clap(long)]
    pub non_fungible: bool,
    /// A template for creating a faucet, allowing users to request test tokens
    #[clap(long)]
    pub faucet: bool,
}
impl InitArgs {
    fn init_template(
        project_dir: &PathBuf,
        json_content: &str,
        example_program: &str,
    ) -> anyhow::Result<()> {
        fs::create_dir_all(&project_dir)?;

        let src_dir = Path::new(&project_dir).join("src");
        fs::create_dir_all(&src_dir)?;

        let json_file_path = Path::new(&project_dir).join("example-program-inputs.json");
        let mut json_file = fs::File::create(json_file_path)?;
        json_file.write_all(json_content.as_bytes())?;

        // Run `cargo init` to initialize the project as a cargo project
        let output = cargo!(&"init", &"--bin", project_dir)?;
        if output.status.success() {
            let main_rs_path = src_dir.join("main.rs");
            let mut main_rs_file = fs::File::create(main_rs_path)?;
            main_rs_file.write_all(example_program.as_bytes())?;

            cargo!(&"add", &"anyhow@1.0", &"lasr_types", &"serde_json@1.0")?;

            //TODO: Eventually add `lasr_rust` as cargo dep for cli access

            println!("Successfully initalized LASR application folder");
        } else {
            eprintln!("Failed to initalized LASR application folder");
            eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
        }

        Ok(())
    }

    pub fn lasr_init(&self) -> anyhow::Result<()> {
        let project_dir = &env::current_dir()?;
        if !project_dir.is_dir() {
            anyhow::bail!(format!("{project_dir:?} is not a valid directory."));
        }

        dbg!(
            "\nUsing project directory at {}",
            project_dir.canonicalize()?.display()
        );

        let keypair_dir = Path::new(&project_dir).join(".lasr");
        fs::create_dir_all(&keypair_dir)?;

        create_new_keypair(keypair_dir);

        match self {
            InitArgs { blank: true, .. } => {
                let json_content = include_str!(
                    "../../../examples/blank/example-program-inputs/blank-create.json"
                );

                let example_program = include_str!("../../../examples/blank/example_program.rs");

                if let Err(e) = Self::init_template(&project_dir, &json_content, &example_program) {
                    eprintln!("Error initializing LASR program: {e:?}");
                    Ok(())
                } else {
                    println!("Initialization completed successfully!");
                    Ok(())
                }
            }
            InitArgs { fungible: true, .. } => {
                let json_content = include_str!(
                    "../../../examples/fungible/example-program-inputs/fungible-create.json"
                );

                let example_program = include_str!("../../../examples/fungible/example_program.rs");

                if let Err(e) = Self::init_template(&project_dir, &json_content, &example_program) {
                    eprintln!("Error initializing LASR program: {e:?}");
                    Ok(())
                } else {
                    println!("Initialization completed successfully!");
                    Ok(())
                }
            }
            InitArgs {
                non_fungible: true, ..
            } => {
                let json_content = include_str!(
                    "../../../examples/non_fungible/example-program-inputs/non-fungible-create.json"
                );

                let example_program =
                    include_str!("../../../examples/non_fungible/example_program.rs");

                if let Err(e) = Self::init_template(&project_dir, &json_content, &example_program) {
                    eprintln!("Error initializing LASR program: {e:?}");
                    Ok(())
                } else {
                    println!("Initialization completed successfully!");
                    Ok(())
                }
            }
            InitArgs { faucet: true, .. } => {
                let json_content = include_str!(
                    "../../../examples/faucet/example-program-inputs/faucet-create.json"
                );

                let example_program = include_str!("../../../examples/faucet/example_program.rs");

                if let Err(e) = Self::init_template(&project_dir, &json_content, &example_program) {
                    eprintln!("Error initializing LASR program: {e:?}");
                    Ok(())
                } else {
                    println!("Initialization completed successfully!");
                    Ok(())
                }
            }
            _ => Err(anyhow::anyhow!("unsupported initialization method")),
        }
    }
}

pub fn create_new_keypair(wallet_path: PathBuf) {
    let mut keypair_path = wallet_path.clone();
    keypair_path.push(KEYPAIR_FILENAME);
    if !keypair_path.exists() {
        println!("Generating new keypair at {keypair_path:?}");
        let wallet_info = Wallet::<HttpClient>::get_info(None, None, None)
            .expect("failed to create new LASR Wallet");
        let contents = serde_json::to_string(&wallet_info).expect("failed to serialize WalletInfo");
        let mut f = fs::File::options()
            .create(true)
            .write(true)
            .open(keypair_path.clone())
            .unwrap();
        writeln!(&mut f, "{contents}").unwrap();
    } else {
        println!("Found existing LASR Wallet!")
    }
}

#[test]
fn test_init_template() {
    let res = crate::lasrctl::cli::InitArgs::lasr_init(&InitArgs {
        blank: true,
        fungible: false,
        non_fungible: false,
        faucet: false,
    });

    assert!(res.is_ok());
}

#[test]
fn test_keypair_json_creation() {
    let project_dir = &env::current_dir().expect("failed to obtain working directory");
    let keypair_dir = Path::new(&project_dir).join(".lasr");
    fs::create_dir_all(&keypair_dir).expect("failed to create keypair dir: .lasr");
    create_new_keypair(keypair_dir);
}

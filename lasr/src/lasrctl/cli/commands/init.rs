use crate::scripts::consts::{KEYPAIR_FILENAME, WALLET_PATH};
use clap::Parser;
use std::{
    env, fs,
    io::{self, Write},
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
        let output = std::process::Command::new("cargo")
            .arg("init")
            .arg("--bin")
            .arg(project_dir)
            .output()?;
        if output.status.success() {
            let main_rs_path = src_dir.join("main.rs");
            let mut main_rs_file = fs::File::create(main_rs_path)?;
            main_rs_file.write_all(example_program.as_bytes())?;
            // TODO: Edit .toml file here.... to include all necessary deps.

            println!("Successfully initalized LASR application folder");
        } else {
            eprintln!("Failed to initalized LASR application folder");
            eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
        }

        Ok(())
    }

    pub async fn lasr_init(init_args: &InitArgs) -> anyhow::Result<()> {
        match init_args {
            InitArgs { blank: true, .. } => {
                let project_dir = &env::current_dir()?;

                if !project_dir.is_dir() {
                    anyhow::bail!(format!("{project_dir:?} is not a valid directory."));
                }

                dbg!(
                    "\nUsing project directory at {}",
                    project_dir.canonicalize()?.display()
                );

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
                let project_dir = &env::current_dir()?;

                if !project_dir.is_dir() {
                    anyhow::bail!(format!("{project_dir:?} is not a valid directory."));
                }

                dbg!(
                    "\nUsing project directory at {}",
                    project_dir.canonicalize()?.display()
                );

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
                let project_dir = &env::current_dir()?;

                if !project_dir.is_dir() {
                    anyhow::bail!(format!("{project_dir:?} is not a valid directory."));
                }

                dbg!(
                    "\nUsing project directory at {}",
                    project_dir.canonicalize()?.display()
                );

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
                let project_dir = &env::current_dir()?;

                if !project_dir.is_dir() {
                    anyhow::bail!(format!("{project_dir:?} is not a valid directory."));
                }

                dbg!(
                    "\nUsing project directory at {}",
                    project_dir.canonicalize()?.display()
                );

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
    println!("Generating new keypair at {keypair_path:?}");
    let mut f = fs::File::options()
        .create(true)
        .write(true)
        .append(true)
        .open(keypair_path.clone())
        .unwrap();
    writeln!(&mut f, "keypair" /*, The generated json keypair */).unwrap();
}

pub fn handle_init_command() {
    let mut wallet_path = env::current_dir().unwrap();
    wallet_path.push(PathBuf::from(WALLET_PATH));
    if !wallet_path.exists() {
        println!("Wallet path not found, would you like to initialize a new lasr wallet at this directory? [Y/n]");
        loop {
            let mut stdin = String::new();
            io::stdin().read_line(&mut stdin).unwrap();
            match stdin.trim().to_lowercase().as_str() {
                "yes" | "y" => {
                    println!("Initializing new wallet at {wallet_path:?}");
                    create_new_keypair(wallet_path);
                    break; // call the inner function
                }
                "no" | "n" => break, /* call the inner function */
                _ => continue,       /* keep looping until user input is valid */
            }
        }
    } else {
        println!("Found existing lasr wallet");
        // call the inner function
    }
}

#[tokio::test]
async fn test_init_template() {
    let res = crate::lasrctl::cli::InitArgs::lasr_init(&InitArgs {
        blank: true,
        fungible: false,
        non_fungible: false,
        faucet: false,
    })
    .await;
    assert!(res.is_ok());
}

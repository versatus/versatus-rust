use crate::scripts::consts::{KEYPAIR_FILENAME, WALLET_PATH};
use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
};

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

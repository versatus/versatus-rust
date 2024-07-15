mod example_program;
use std::{io::Read, path::PathBuf};

pub fn construct_init_template(
    blank: bool,
    fungible: bool,
    non_fungible: bool,
    faucet: bool,
) -> Result<String, anyhow::Error> {
    match (blank, fungible, non_fungible, faucet) {
        (true, false, false, false) => {
            let path = PathBuf::from("src/examples/blank/example-program-inputs/blank-create.json");
            let mut input_string = String::new();
            let mut f = std::fs::OpenOptions::new()
                .read(true)
                .write(false)
                .truncate(false)
                .append(false)
                .create(false)
                .open(path)
                .expect("unable to open json file");

            f.read_to_string(&mut input_string)
                .expect("unable to read json file contents to string");

            let init_template = serde_json::to_string_pretty(&input_string)
                .map_err(|e| anyhow::anyhow!(e.to_string()))?;

            Ok(init_template)
        }
        _ => todo!(),
    }
}

#[cfg(test)]
#[tokio::test]
async fn test_init_template() {
    let res = construct_init_template(true, false, false, false).unwrap();
    dbg!(&res);
}

use anyhow::Result;
use versatus_rust::versatus_rust::{ComputeInputs, ComputeOutputs, ComputeTransaction};

fn main() -> Result<()> {
    // Gather the smart contract inputs from stdin
    let input = ComputeInputs::gather()?;

    // Do contract stuff to generate proposed transactions
    let mut transactions: Vec<ComputeTransaction> = vec![];
    let amount_each: u64 =
        input.application_input.amount / input.application_input.recipients.len() as u64;
    for recipient in input.application_input.recipients.iter() {
        let txn = ComputeTransaction {
            recipient: recipient.to_string(),
            amount: amount_each,
        };
        transactions.push(txn);
    }

    // Create output object containing proposed transactions
    let output = ComputeOutputs { transactions };

    // Write the smart contract results/transactions to stdout
    output.commit()?;

    Ok(())
}

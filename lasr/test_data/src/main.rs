//! A minimalistic main function for a Rust LASR program.
//! A Program that takes in lasr_type::Inputs, handles the call based on the program method, and produces necessary lasr_types::Outputs to be processed by protocol
use anyhow::Ok;
use lasr_rust::lasrctl::builders::program::{CreateTransactionInputs, Program};
use lasr_types::*;
use std::collections::BTreeMap;

pub struct BlankProgram {
    _program: Program<Inputs>,
}

impl BlankProgram {
    pub fn new(_inputs: Inputs) -> Program<Inputs> {
        Program::new()
    }

    pub fn hello(inputs: Inputs) -> Result<String, anyhow::Error> {
        let transaction = &inputs.transaction;
        let txn_inputs: CreateTransactionInputs =
            serde_json::from_str(&transaction.inputs().as_str())
                .map_err(|e| anyhow::anyhow!("failed to deserialize txInputs: {e}"))?;

        let name = serde_json::to_string(&format!("{:?} World", txn_inputs.name))
            .map_err(|e| anyhow::anyhow!("failed to serialize txn metadata: {e:?}"))?;
        let current_time = std::time::SystemTime::now();

        let update_val =
            serde_json::to_string(&format!("Hello, {name}! The time is {current_time:?}!"))?;

        let mut update_map: BTreeMap<String, String> = BTreeMap::new();
        update_map.insert("data".to_string(), update_val);

        let program_update_field = ProgramUpdateFieldBuilder::new()
            .field(ProgramField::Data)
            .value(ProgramFieldValue::Data(DataValue::Extend(update_map)))
            .build()?;

        let program_update =
            ProgramUpdate::new(AddressOrNamespace::This, [program_update_field].to_vec());

        let update_instruction = UpdateInstructionBuilder::new()
            .add_update(TokenOrProgramUpdate::ProgramUpdate(program_update))
            .build();

        let outputs: Outputs = OutputsBuilder::new()
            .inputs(inputs)
            .add_instruction(lasr_types::Instruction::Update(update_instruction))
            .build()
            .map_err(|e| anyhow::anyhow!("failed to build computeOutputs: {e:?}"))?;

        let obj = serde_json::to_string_pretty(&outputs).unwrap();

        Ok(obj)
    }

    pub fn start(inputs: Inputs) -> Result<String, anyhow::Error> {
        let blank = Program::new();
        let outputs = blank
            .execute_method(&inputs)
            .map_err(|e| anyhow::anyhow!("failed to update program: {e:?}"))?;

        Ok(outputs)
    }
}

#[allow(dead_code)]
fn main() -> anyhow::Result<()> {
    if let Err(e) = Program::run() {
        anyhow::bail!("program failed: {e:?}")
    }
    Ok(())
}

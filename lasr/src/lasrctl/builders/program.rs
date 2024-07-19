use std::str::FromStr;

use anyhow::Ok;
use lasr_types::*;
use serde::{Deserialize, Serialize};

pub struct Program {
    method_strategies: MethodStrategy,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MethodStrategy {
    Approve,
    Create,
    Update,
}

impl MethodStrategy {
    pub fn execute_method(&self, inputs: Inputs) -> Result<String, anyhow::Error> {
        let op = &inputs.op;
        let strategy: Self = serde_json::from_str(&op)
            .map_err(|e| anyhow::anyhow!("failed to serialize {op:?} from computeInputs: {e:?}"))?;

        let res = match strategy {
            MethodStrategy::Approve => approve_program(inputs)
                .map_err(|e| anyhow::anyhow!("failed to approve program: {e:?}"))?,
            MethodStrategy::Create => create_program(inputs)
                .map_err(|e| anyhow::anyhow!("failed to create program: {e:?}"))?,
            MethodStrategy::Update => todo!(),
        };
        Ok(res)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(
    rename(serialize = "transactionInputs", deserialize = "transactionInputs"),
    rename_all = "camelCase"
)]
pub struct TransactionInputs {
    symbol: String,
    name: String,
    total_supply: Option<String>,
    initialized_supply: Option<String>,
    data: Option<String>,
    metadata: Option<String>,
    linked_programs: Option<Vec<Address>>,
}

pub fn approve_program(inputs: Inputs) -> Result<String, anyhow::Error> {
    let transaction = &inputs.transaction;
    let txn_inputs = transaction.inputs();
    let program_id = transaction.program_id();
    let program_address = AddressOrNamespace::Address(program_id);
    let caller = Address::from(transaction.from());
    let update_val = TokenFieldValue::Data(DataValue::Insert("approvals".to_string(), txn_inputs));

    let update = TokenUpdateFieldBuilder::new()
        .field(TokenField::Approvals)
        .value(update_val)
        .build()
        .map_err(|e| anyhow::anyhow!("failed to build update_field: {e:?}"))?;

    let token_update = TokenUpdateBuilder::new()
        .token(AddressOrNamespace::Address(caller))
        .account(program_address)
        .add_update(update)
        .build()
        .map_err(|e| anyhow::anyhow!("failed to build TokenUpdate for {caller}: {e:?}"))?;

    let token_pgrm_update = TokenOrProgramUpdate::TokenUpdate(token_update);

    let update_instruction = UpdateInstructionBuilder::new()
        .add_update(token_pgrm_update)
        .build();

    let outputs: Outputs = OutputsBuilder::new()
        .inputs(inputs)
        .add_instruction(lasr_types::Instruction::Update(update_instruction))
        .build()
        .map_err(|e| anyhow::anyhow!("failed to build computeOutputs: {e:?}"))?;

    let obj = serde_json::to_string_pretty(&outputs).unwrap();

    Ok(obj)
}

pub fn create_program(inputs: Inputs) -> Result<String, anyhow::Error> {
    let transaction = &inputs.transaction;
    let from = transaction.from();
    let txn_inputs: TransactionInputs = serde_json::from_str(transaction.inputs().as_str())?;

    let symbol = txn_inputs.symbol;
    let name = txn_inputs.name;
    let total_supply = txn_inputs.total_supply.unwrap_or_default();
    let initialized_supply = txn_inputs.initialized_supply.unwrap_or_default();

    let metadata_str = serde_json::to_string(&(symbol, name, total_supply.clone()))
        .map_err(|e| anyhow::anyhow!("failed to serialize txn metadata: {e:?}"))?;
    let update_field_val =
        ProgramFieldValue::Metadata(MetadataValue::Insert("metadata".to_string(), metadata_str));

    let create_dist_instructions = CreateInstructionBuilder::new()
        .program_owner(from)
        .initialized_supply(U256::from_str(&initialized_supply)?)
        .total_supply(U256::from_str(&total_supply)?)
        .program_id(AddressOrNamespace::This)
        .program_namespace(AddressOrNamespace::This)
        .build()
        .map_err(|e| anyhow::anyhow!("failed to build create instructions: {e:?}"))?;

    let program_metadata = ProgramUpdateFieldBuilder::new()
        .field(ProgramField::Metadata)
        .value(update_field_val)
        .build()
        .map_err(|e| anyhow::anyhow!("failed to build program update field: {e:?}"))?;

    let update_instruction = UpdateInstructionBuilder::new()
        .add_update(TokenOrProgramUpdate::ProgramUpdate(
            ProgramUpdateBuilder::new()
                .account(AddressOrNamespace::This)
                .add_update(program_metadata)
                .build()
                .map_err(|e| anyhow::anyhow!("failed to build program update: {e:?}"))?,
        ))
        .build();

    let outputs: Outputs = OutputsBuilder::new()
        .inputs(inputs)
        .add_instruction(lasr_types::Instruction::Create(create_dist_instructions))
        .add_instruction(lasr_types::Instruction::Update(update_instruction))
        .build()
        .map_err(|e| anyhow::anyhow!("failed to build computeOutputs: {e:?}"))?;

    let obj = serde_json::to_string_pretty(&outputs).unwrap();

    Ok(obj)
}

pub fn update_program(inputs: Inputs) -> Result<String, anyhow::Error> {
    let transaction = &inputs.transaction;
    let txn_inputs: TransactionInputs = serde_json::from_str(transaction.inputs().as_str())?;
    let data = txn_inputs.data;
    let metadata = txn_inputs.metadata;
    let linked_programs = txn_inputs.linked_programs;

    let mut program_updates = Vec::new();

    if let Some(metadata) = metadata {
        let update_field_val =
            ProgramFieldValue::Metadata(MetadataValue::Insert("metadata".to_string(), metadata));

        let update_field = ProgramUpdateFieldBuilder::new()
            .field(ProgramField::Metadata)
            .value(update_field_val)
            .build()
            .map_err(|e| anyhow::anyhow!("failed to build program update field: {e:?}"))?;

        program_updates.push(update_field)
    };

    if let Some(data) = data {
        let update_field_val = ProgramFieldValue::Data(DataValue::Insert("data".to_string(), data));

        let update_field = ProgramUpdateFieldBuilder::new()
            .field(ProgramField::Data)
            .value(update_field_val)
            .build()
            .map_err(|e| anyhow::anyhow!("failed to build program update field: {e:?}"))?;

        program_updates.push(update_field)
    };

    while let mut linked_program = linked_programs.clone().unwrap_or_default().iter() {
        let update_field_val = ProgramFieldValue::LinkedPrograms(LinkedProgramsValue::Insert(
            *linked_program
                .next()
                .expect("failed to insert linked program"),
        ));

        let update_field = ProgramUpdateFieldBuilder::new()
            .field(ProgramField::LinkedPrograms)
            .value(update_field_val)
            .build()
            .map_err(|e| anyhow::anyhow!("failed to build program update field: {e:?}"))?;

        program_updates.push(update_field);
    }

    let update_instruction = UpdateInstructionBuilder::new()
        .add_update(TokenOrProgramUpdate::ProgramUpdate(
            ProgramUpdateBuilder::new()
                .account(AddressOrNamespace::This)
                .extend_updates(program_updates)
                .build()
                .map_err(|e| anyhow::anyhow!("failed to build program update: {e:?}"))?,
        ))
        .build();

    let outputs: Outputs = OutputsBuilder::new()
        .inputs(inputs)
        .add_instruction(lasr_types::Instruction::Update(update_instruction))
        .build()
        .map_err(|e| anyhow::anyhow!("failed to build computeOutputs: {e:?}"))?;

    let obj = serde_json::to_string_pretty(&outputs).unwrap();

    Ok(obj)
}

#[cfg(test)]
#[tokio::test]
async fn test_approval() -> Result<(), anyhow::Error> {
    use crate::examples::blank::example_program::init_program;

    let method = MethodStrategy::Approve;
    let template_str =
        include_str!("../../examples/blank/example-program-inputs/blank-create.json");

    let map: Inputs = serde_json::from_str(&template_str)
        .map_err(|e| anyhow::anyhow!("failed to destructure json template: {e:?}"))?;
    // A JSON object representative of a LASR Program
    let program = init_program(method, map).unwrap();
    println!("{program:?}");
    Ok(())
}

#[tokio::test]
async fn test_create() -> Result<(), anyhow::Error> {
    use crate::examples::blank::example_program::init_program;

    let method = MethodStrategy::Create;
    let template_str =
        include_str!("../../examples/blank/example-program-inputs/blank-create.json");

    let map: Inputs = serde_json::from_str(&template_str)
        .map_err(|e| anyhow::anyhow!("failed to destructure json template: {e:?}"))?;

    let program = init_program(method, map).unwrap();
    println!("{program:?}");
    Ok(())
}

#[tokio::test]
async fn test_update() -> Result<(), anyhow::Error> {
    use crate::examples::blank::example_program::init_program;

    let method = MethodStrategy::Approve;
    let template_str =
        include_str!("../../examples/blank/example-program-inputs/blank-update.json");

    let map: Inputs = serde_json::from_str(&template_str)
        .map_err(|e| anyhow::anyhow!("failed to destructure json template: {e:?}"))?;

    let program = init_program(method, map).unwrap();
    println!("{program:?}");
    Ok(())
}

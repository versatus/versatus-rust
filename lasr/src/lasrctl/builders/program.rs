use std::{collections::HashMap, io::Read, str::FromStr};

use anyhow::Ok;
use lasr_types::*;
use serde::{Deserialize, Serialize};

pub struct Program<Inputs> {
    method_strategies: HashMap<String, Box<dyn Fn(&Inputs) -> String>>,
}

impl Program<Inputs> {
    pub fn new() -> Self {
        let mut program = Program {
            method_strategies: HashMap::new(),
        };

        // Register default methods
        program.register_contract_method("approve".to_string(), Box::new(Program::approve));
        program.register_contract_method("create".to_string(), Box::new(Program::create));
        program.register_contract_method("update".to_string(), Box::new(Program::update));

        program
    }

    fn register_contract_method(
        &mut self,
        operation: String,
        method: Box<dyn Fn(&Inputs) -> String>,
    ) {
        self.method_strategies.insert(operation, method);
    }

    fn approve(inputs: &Inputs) -> String {
        approve_program(inputs.clone())
            .map_err(|e| anyhow::anyhow!("failed to approve lasr program: {e:?}"))
            .unwrap()
    }

    fn create(inputs: &Inputs) -> String {
        create_program(inputs.clone())
            .map_err(|e| anyhow::anyhow!("failed to create lasr program: {e:?}"))
            .unwrap()
    }

    fn update(inputs: &Inputs) -> String {
        update_program(inputs.clone())
            .map_err(|e| anyhow::anyhow!("failed to update lasr program: {e:?}"))
            .unwrap()
    }

    pub fn execute_method(&self, inputs: &Inputs) -> Result<String, anyhow::Error> {
        match self.method_strategies.get(&inputs.op) {
            Some(strategy) => Ok(strategy(inputs)),
            None => Err(anyhow::anyhow!("Invalid method strategy")),
        }
    }

    pub fn start(&self, compute_inputs: &Inputs) -> Result<String, anyhow::Error> {
        self.execute_method(compute_inputs)
    }

    pub fn run() -> anyhow::Result<()> {
        let mut input = String::new();
        std::io::stdin()
            .read_to_string(&mut input)
            .map_err(|e| anyhow::anyhow!("error while reading stdin to string: {e:?}"))?;

        let parsed_data: Inputs = serde_json::from_str(&input).map_err(|e| {
            anyhow::anyhow!("error serializing stdin inputs to lasr_type Inputs: {e:?}")
        })?;
        let program = Program::new();
        let result = program
            .start(&parsed_data)
            .map_err(|e| anyhow::anyhow!("error while starting LASR program: {e:?}"))?;

        println!("{}", result);
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MethodStrategy {
    Approve,
    Create,
    Update,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(
    rename(serialize = "transactionInputs", deserialize = "transactionInputs"),
    rename_all = "camelCase"
)]
pub struct CreateTransactionInputs {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub total_supply: Option<String>,
    pub initialized_supply: Option<String>,
    pub img_url: Option<String>,
    pub payment_program_address: Option<String>,
    pub price: Option<String>,
    pub collection: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(
    rename(serialize = "transactionInputs", deserialize = "transactionInputs"),
    rename_all = "camelCase"
)]
/// Structure of transactionInputs specifically for the update method strategy.
pub struct UpdateTransactionInputs {
    data: Option<HashMap<String, String>>,
    metadata: Option<Metadata>,
    linked_programs: Option<Vec<Address>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    symbol: Option<String>,
    name: Option<String>,
    total_supply: Option<String>,
    initialized_supply: Option<String>,
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
    let txn_inputs: CreateTransactionInputs = serde_json::from_str(transaction.inputs().as_str())?;

    let total_supply = txn_inputs.total_supply.unwrap_or_default();
    let initialized_supply = txn_inputs.initialized_supply.unwrap_or_default();

    let metadata_str =
        serde_json::to_string(&(txn_inputs.symbol, txn_inputs.name, total_supply.clone()))
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
    let txn_inputs: UpdateTransactionInputs = serde_json::from_str(transaction.inputs().as_str())?;

    let mut program_updates = Vec::new();

    if let Some(metadata) = txn_inputs.metadata {
        let metadata = serde_json::to_string(&metadata)?;
        let update_field_val =
            ProgramFieldValue::Metadata(MetadataValue::Insert("metadata".to_string(), metadata));

        let update_field = ProgramUpdateFieldBuilder::new()
            .field(ProgramField::Metadata)
            .value(update_field_val)
            .build()
            .map_err(|e| anyhow::anyhow!("failed to build program update field: {e:?}"))?;

        program_updates.push(update_field)
    };

    if let Some(data) = txn_inputs.data {
        let data = serde_json::to_string(&data)?;
        let update_field_val = ProgramFieldValue::Data(DataValue::Insert("data".to_string(), data));

        let update_field = ProgramUpdateFieldBuilder::new()
            .field(ProgramField::Data)
            .value(update_field_val)
            .build()
            .map_err(|e| anyhow::anyhow!("failed to build program update field: {e:?}"))?;

        program_updates.push(update_field)
    };

    if txn_inputs.linked_programs.is_some() {
        for linked_program in txn_inputs.linked_programs.unwrap() {
            let update_field_val =
                ProgramFieldValue::LinkedPrograms(LinkedProgramsValue::Insert(linked_program));
            let update_field = ProgramUpdateFieldBuilder::new()
                .field(ProgramField::LinkedPrograms)
                .value(update_field_val)
                .build()
                .map_err(|e| anyhow::anyhow!("failed to build program update field: {e:?}"))?;

            program_updates.push(update_field);
        }
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
    let template_str =
        include_str!("../../examples/fungible/example-program-inputs/fungible-approve.json");

    let compute_inputs: Inputs = serde_json::from_str(&template_str)
        .map_err(|e| anyhow::anyhow!("failed to destructure json template: {e:?}"))?;

    let program = Program::new();
    let result = program
        .start(&compute_inputs)
        .map_err(|e| e.to_string())
        .unwrap();

    let json_output = serde_json::to_string(&result)?;
    println!("{json_output}");

    Ok(())
}

#[tokio::test]
async fn test_create() -> Result<(), anyhow::Error> {
    let template_str =
        include_str!("../../examples/fungible/example-program-inputs/fungible-create.json");

    let compute_inputs: Inputs = serde_json::from_str(&template_str)
        .map_err(|e| anyhow::anyhow!("failed to destructure json template: {e:?}"))?;

    let program = Program::new();
    let result = program
        .start(&compute_inputs)
        .map_err(|e| e.to_string())
        .unwrap();

    let json_output = serde_json::to_string(&result)?;
    println!("{json_output}");

    Ok(())
}

#[tokio::test]
async fn test_update() -> Result<(), anyhow::Error> {
    let template_str =
        include_str!("../../examples/fungible/example-program-inputs/fungible-update.json");

    let compute_inputs: Inputs = serde_json::from_str(&template_str)
        .map_err(|e| anyhow::anyhow!("failed to destructure json template: {e:?}"))?;

    let program = Program::new();
    let result = program
        .start(&compute_inputs)
        .map_err(|e| e.to_string())
        .unwrap();

    let json_output = serde_json::to_string(&result)?;
    println!("{json_output}");

    Ok(())
}

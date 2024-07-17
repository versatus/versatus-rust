use anyhow::Ok;
use lasr_types::*;

pub struct Program {
    method_strategies: MethodStrategies,
}

pub enum MethodStrategies {
    Approve,
    Create,
    Update,
}

pub fn approve_program(inputs: Inputs) -> Result<serde_json::Value, anyhow::Error> {
    let transaction = &inputs.transaction;
    let txn_inputs = transaction.inputs();
    let program_id = transaction.program_id();
    let program_address = AddressOrNamespace::Address(program_id);
    let caller = Address::from(transaction.from());

    let update = TokenUpdateFieldBuilder::new()
        .field(TokenField::Approvals)
        .value(
            serde_json::from_str(&txn_inputs)
                .map_err(|e| anyhow::anyhow!("failed to parse txn_inputs: {e:?}"))?,
        )
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

    let res = serde_json::to_value(&outputs)
        .map_err(|e| anyhow::anyhow!("failed to deserialize outputs to JSON: {e:?}"))?;

    Ok(res)
}

#[cfg(test)]
#[tokio::test]
async fn test_approval() -> Result<(), anyhow::Error> {
    use crate::examples::blank::example_program::init_program;

    let method = MethodStrategies::Approve;
    let template_str =
        include_str!("../../examples/blank/example-program-inputs/blank-create.json");

    let map: Inputs = serde_json::from_str(&template_str)
        .map_err(|e| anyhow::anyhow!("failed to destructure json template: {e:?}"))?;
    // A JSON object representative of a LASR Program
    let program = init_program(method, map);
    dbg!(&program);
    Ok(())
}

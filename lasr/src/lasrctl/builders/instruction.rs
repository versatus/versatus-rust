use anyhow::Result;
use lasr_types::{
    Address, AddressOrNamespace, BurnInstruction, BurnInstructionBuilder, CreateInstruction,
    CreateInstructionBuilder, InnerInstruction, ProgramField, ProgramFieldValue,
    ProgramUpdateField, ProgramUpdateFieldBuilder, TokenDistribution, TokenDistributionBuilder,
    TokenField, TokenFieldValue, TokenOrProgramUpdate, TokenUpdateField, TokenUpdateFieldBuilder,
    TransferInstruction, TransferInstructionBuilder, UpdateInstruction, UpdateInstructionBuilder,
    U256,
};
use serde::Serialize;

/// Instruction builder for CreateInstruction
pub fn create_instruction_builder(
    program_namespace: AddressOrNamespace,
    program_id: AddressOrNamespace,
    program_owner: Address,
    total_supply: U256,
    initialized_supply: U256,
    distribution: Vec<TokenDistribution>,
) -> Result<CreateInstruction> {
    CreateInstructionBuilder::new()
        .program_namespace(program_namespace)
        .program_id(program_id)
        .program_owner(program_owner)
        .total_supply(total_supply)
        .initialized_supply(initialized_supply)
        .extend_token_distributions(distribution)
        .build()
        .map_err(|e| anyhow::anyhow!("{e:?}"))
}

pub fn token_update_field_builder(
    field: TokenField,
    value: TokenFieldValue,
) -> Result<TokenUpdateField> {
    TokenUpdateFieldBuilder::new()
        .field(field)
        .value(value)
        .build()
        .map_err(|e| anyhow::anyhow!("{e:?}"))
}

pub fn program_update_field_builder(
    field: ProgramField,
    value: ProgramFieldValue,
) -> Result<ProgramUpdateField> {
    ProgramUpdateFieldBuilder::new()
        .field(field)
        .value(value)
        .build()
        .map_err(|e| anyhow::anyhow!("{e:?}"))
}

/// Instruction builder for UpdateInstruction
pub fn update_instruction_builder(updates: Vec<TokenOrProgramUpdate>) -> UpdateInstruction {
    let updates = UpdateInstructionBuilder::new()
        .extend_updates(updates)
        .build();
    if updates.updates().is_empty() {
        eprintln!("Warning: No updates were found when building UpdateInstruction. UpdateInstruction is empty.");
    }
    updates
}

/// Instruction builder for TransferInstruction
pub fn transfer_instruction_builder(
    token: Address,
    from: AddressOrNamespace,
    to: AddressOrNamespace,
    amount: Option<U256>,
    ids: Vec<U256>,
) -> Result<TransferInstruction> {
    TransferInstructionBuilder::new()
        .token(token)
        .from(from)
        .to(to)
        .amount(amount.unwrap_or_default())
        .extend_ids(ids)
        .build()
        .map_err(|e| anyhow::anyhow!("{e:?}"))
}

/// Instruction builder for BurnInstruction
pub fn burn_instruction_builder(
    caller: Address,
    program_id: AddressOrNamespace,
    token: Address,
    from: AddressOrNamespace,
    amount: Option<U256>,
    token_ids: Vec<U256>,
) -> Result<BurnInstruction> {
    BurnInstructionBuilder::new()
        .caller(caller)
        .program_id(program_id)
        .token(token)
        .from(from)
        .amount(amount.unwrap_or_default())
        .extend_token_ids(token_ids)
        .build()
        .map_err(|e| anyhow::anyhow!("{e:?}"))
}

pub fn token_distr_builder(
    program_id: AddressOrNamespace,
    to: AddressOrNamespace,
    amount: Option<U256>,
    token_ids: Vec<U256>,
    update_fields: Vec<TokenUpdateField>,
) -> Result<TokenDistribution> {
    TokenDistributionBuilder::new()
        .program_id(program_id)
        .to(to)
        .amount(amount.unwrap_or_default())
        .extend_token_ids(token_ids)
        .extend_update_fields(update_fields)
        .build()
        .map_err(|e| anyhow::anyhow!("{e:?}"))
}

pub fn instruction_to_json<I: InnerInstruction + Serialize>(inst: &I) -> Result<String> {
    serde_json::to_string(inst).map_err(|e| anyhow::anyhow!("{e:?}"))
}

#[cfg(test)]
#[tokio::test]
async fn test_create_inst_builder() {
    let program_namespace = AddressOrNamespace::This;
    let program_id = AddressOrNamespace::Address(Address::from([0; 20]));
    let program_owner = Address::from([0; 20]);
    let total_supply = lasr_types::U256::from(0);
    let initialized_supply = lasr_types::U256::from(0);
    let distribution = vec![TokenDistribution::default()];

    let create_instruction = create_instruction_builder(
        program_namespace,
        program_id,
        program_owner,
        total_supply,
        initialized_supply,
        distribution,
    )
    .map_err(|e| e.to_string())
    .unwrap();

    let res = serde_json::to_string(&create_instruction)
        .map_err(|e| e.to_string())
        .unwrap();

    dbg!(&res);
}

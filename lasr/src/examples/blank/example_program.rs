use lasr_types::*;

use crate::lasrctl::builders::program::{approve_program, MethodStrategies};

pub fn init_program(
    method: MethodStrategies,
    inputs: Inputs,
) -> Result<serde_json::Value, anyhow::Error> {
    match method {
        MethodStrategies::Approve => approve_program(inputs),
        MethodStrategies::Create => todo!(),
        MethodStrategies::Update => todo!(),
    }
}

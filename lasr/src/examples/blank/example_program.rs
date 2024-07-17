use lasr_types::*;

use crate::lasrctl::builders::program::{approve_program, MethodStrategies};

pub fn init_program(method: MethodStrategies, inputs: Inputs) -> Result<String, anyhow::Error> {
    match method {
        MethodStrategies::Approve => approve_program(inputs),
        MethodStrategies::Create => todo!(),
        MethodStrategies::Update => todo!(),
    }
}

use lasr_types::*;

use crate::lasrctl::builders::program::{approve_program, create_program, MethodStrategy};

pub fn init_program(method: MethodStrategy, inputs: Inputs) -> Result<String, anyhow::Error> {
    match method {
        MethodStrategy::Approve => approve_program(inputs),
        MethodStrategy::Create => create_program(inputs),
        MethodStrategy::Update => todo!(),
    }
}

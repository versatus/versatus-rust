use anyhow::Ok;
use lasr_types::*;
use serde::{Deserialize, Serialize};

use crate::lasrctl::builders::program::{
    approve_program, create_program, update_program, MethodStrategy, Program,
};

pub fn init_program(method: MethodStrategy, inputs: Inputs) -> Result<String, anyhow::Error> {
    match method {
        MethodStrategy::Approve => approve_program(inputs),
        MethodStrategy::Create => create_program(inputs),
        MethodStrategy::Update => update_program(inputs),
    }
}

pub struct Batman {
    program: Program<Inputs>,
}

impl Batman {
    pub fn hello(inputs: Inputs) -> Result<String, anyhow::Error> {
        let batman = Program::new();
        let outputs = batman
            .execute_method(&inputs)
            .map_err(|e| anyhow::anyhow!("failed to update program: {e:?}"))?;

        Ok(outputs)
    }
}

use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Clone, Serialize, Deserialize)]
pub struct Variable {
    pub index: usize,
    pub value: BigInt,
}

#[derive(Serialize, Deserialize)]
pub enum Operation {
    Add,
    Mul,
    Hash,
}

#[derive(Serialize, Deserialize)]
pub struct Constraint {
    pub left: Vec<(Variable, BigInt)>,
    pub right: Vec<(Variable, BigInt)>,
    pub output: Vec<(Variable, BigInt)>,
    pub operation: Operation,
}

#[derive(Serialize, Deserialize)]
pub struct R1CS {
    pub variables: Vec<Variable>,
    pub constraints: Vec<Constraint>,
}

impl R1CS {
    pub fn new() -> Self {
        R1CS {
            variables: Vec::new(),
            constraints: Vec::new(),
        }
    }

    pub fn add_constraint(
        &mut self,
        left: Vec<(Variable, BigInt)>,
        right: Vec<(Variable, BigInt)>,
        output: Vec<(Variable, BigInt)>,
        operation: Operation,
    ) {
        let constraint = Constraint {
            left,
            right,
            output,
            operation,
        };
        self.constraints.push(constraint);
    }

    pub fn is_satisfied<F>(&self, apply_hash: F) -> bool
    where
        F: Fn(&BigInt, &BigInt) -> BigInt,
    {
        for constraint in &self.constraints {
            let left_value: BigInt = constraint
                .left
                .iter()
                .map(|(var, coeff)| &var.value * coeff)
                .sum();
            let right_value: BigInt = constraint
                .right
                .iter()
                .map(|(var, coeff)| &var.value * coeff)
                .sum();
            let output_value: BigInt = constraint
                .output
                .iter()
                .map(|(var, coeff)| &var.value * coeff)
                .sum();

            return match constraint.operation {
                Operation::Add => left_value + right_value == output_value,
                Operation::Mul => left_value * right_value == output_value,
                Operation::Hash => {
                    let computed_hash = apply_hash(&left_value, &right_value);
                    output_value == computed_hash
                }
            };
        }
        true
    }

    pub fn save_to_binary(&self, file_name: &str) {
        let mut file = File::create(file_name).expect("Unable to create file");
        let data = bincode::serialize(&self).expect("Unable to serialize R1CS");
        file.write_all(&data).expect("Unable to write data");
    }
}

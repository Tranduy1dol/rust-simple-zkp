use crate::hash_function::HashFunction;
use crate::r1cs::{Operation, Variable, R1CS};
use num_bigint::BigInt;
use std::io::Write;

pub enum Gate {
    Add(usize, usize, usize),  // Add: input1, input2, output
    Mul(usize, usize, usize),  // Mul: input1, input2, output
    Hash(usize, usize, usize), // Hash: input1, input2, output
}

pub struct Circuit {
    hash_function: Option<Box<dyn HashFunction>>, // User can use custom hash function here
    inputs: Vec<BigInt>,
    gates: Vec<Gate>,
    outputs: Vec<BigInt>,
}

impl Circuit {
    pub fn new(hash_function: Option<Box<dyn HashFunction>>) -> Self {
        Circuit {
            hash_function,
            inputs: Vec::new(),
            gates: Vec::new(),
            outputs: Vec::new(),
        }
    }

    pub fn add_input(&mut self, input: BigInt) -> usize {
        let index = self.inputs.len();
        self.inputs.push(input);
        index
    }

    pub fn add_gate(&mut self, gate: Gate) {
        self.gates.push(gate);
    }

    pub fn set_output(&mut self, output: BigInt) {
        self.outputs.push(output);
    }

    pub fn apply_hash(&self, a: &BigInt, b: &BigInt) -> BigInt {
        if let Some(ref hash_function) = self.hash_function {
            hash_function.hash(a, b)
        } else {
            a + b
        }
    }

    pub fn get_input(&self, index: usize) -> Option<&BigInt> {
        self.inputs.get(index)
    }

    pub fn generate_proof(&self, proof_file: &str) {
        let mut r1cs = R1CS::new();
        r1cs.variables = self
            .inputs
            .iter()
            .enumerate()
            .map(|(i, v)| Variable {
                index: i,
                value: v.clone(),
            })
            .collect();

        for gate in &self.gates {
            match gate {
                Gate::Add(a, b, output) => r1cs.add_constraint(
                    vec![(r1cs.variables[*a].clone(), BigInt::from(1))],
                    vec![(r1cs.variables[*b].clone(), BigInt::from(1))],
                    vec![(r1cs.variables[*output].clone(), BigInt::from(1))],
                    Operation::Add,
                ),
                Gate::Mul(a, b, output) => r1cs.add_constraint(
                    vec![(r1cs.variables[*a].clone(), BigInt::from(1))],
                    vec![(r1cs.variables[*b].clone(), BigInt::from(1))],
                    vec![(r1cs.variables[*output].clone(), BigInt::from(1))],
                    Operation::Mul,
                ),
                Gate::Hash(a, b, output) => {
                    let computed_hash = self.apply_hash(&self.inputs[*a], &self.inputs[*b]);
                    r1cs.variables[*output].value = computed_hash.clone();

                    r1cs.add_constraint(
                        vec![(r1cs.variables[*a].clone(), BigInt::from(1))],
                        vec![(r1cs.variables[*b].clone(), BigInt::from(1))],
                        vec![(r1cs.variables[*output].clone(), BigInt::from(1))],
                        Operation::Hash,
                    )
                }
            }
        }

        let is_valid = r1cs.is_satisfied(|a, b| {
            if let Some(ref hash_function) = self.hash_function {
                hash_function.hash(a, b)
            } else {
                a + b
            }
        });

        let mut file = std::fs::File::create(proof_file).expect("Unable to create proof file");
        file.write_all(&[is_valid as u8])
            .expect("Unable to write data");
    }

    pub fn verify_proof(&self, proof_file: &str) -> bool {
        let proof_data = std::fs::read(proof_file).expect("Unable to read proof file");
        proof_data[0] == 1
    }
}

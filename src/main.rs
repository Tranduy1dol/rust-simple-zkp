use crate::circuit::Circuit;
use crate::hash_function::CustomHash;
use crate::merkle::MerkleTree;
use num_bigint::ToBigInt;

mod circuit;
mod hash_function;
mod merkle;
mod r1cs;

fn additional_proof() {
    let mut circuit = Circuit::new(None);

    let input1 = circuit.add_input(1.to_bigint().unwrap());
    let input2 = circuit.add_input(2.to_bigint().unwrap());

    let output_index = circuit.add_input(3.to_bigint().unwrap());
    circuit.add_gate(circuit::Gate::Add(input1, input2, output_index));
    circuit.set_output(3.to_bigint().unwrap());

    circuit.generate_proof("additional_proof.bin");
    let is_valid = circuit.verify_proof("additional_proof.bin");
    eprintln!("is_valid = {:#?}", is_valid);
}

fn multiplication_proof() {
    let mut circuit = Circuit::new(None);

    let input1 = circuit.add_input(3.to_bigint().unwrap());
    let input2 = circuit.add_input(2.to_bigint().unwrap());

    let output_index = circuit.add_input(6.to_bigint().unwrap());
    circuit.add_gate(circuit::Gate::Mul(input1, input2, output_index));
    circuit.set_output(3.to_bigint().unwrap());

    circuit.generate_proof("multiplication_proof.bin");
    let is_valid = circuit.verify_proof("multiplication_proof.bin");
    eprintln!("is_valid = {:#?}", is_valid);
}

fn merkle_tree_proof() {
    let transaction = vec![
        1.to_bigint().unwrap(),
        2.to_bigint().unwrap(),
        3.to_bigint().unwrap(),
        4.to_bigint().unwrap(),
        5.to_bigint().unwrap(),
    ];

    let merkle_tree = MerkleTree::new(transaction.clone(), CustomHash);
    let leaf_index = 2;
    let leaf_value = transaction[leaf_index].clone();
    let merkle_path = merkle_tree.merkle_path(leaf_index);

    let mut circuit = Circuit::new(Some(Box::new(CustomHash)));
    let leaf_index_var = circuit.add_input(leaf_value);
    let mut current_hash_index = leaf_index_var;

    for (sibling_hash, is_left) in merkle_path {
        let sibling_index_var = circuit.add_input(sibling_hash.clone());

        let new_hash_value = if is_left {
            circuit.apply_hash(
                circuit
                    .get_input(sibling_index_var)
                    .expect("Invalid input index"),
                circuit
                    .get_input(current_hash_index)
                    .expect("Invalid input index"),
            )
        } else {
            circuit.apply_hash(
                circuit
                    .get_input(current_hash_index)
                    .expect("Invalid input index"),
                circuit
                    .get_input(sibling_index_var)
                    .expect("Invalid input index"),
            )
        };

        let new_hash_index = circuit.add_input(new_hash_value.clone());
        circuit.set_output(new_hash_value.clone());

        circuit.add_gate(if is_left {
            circuit::Gate::Hash(sibling_index_var, current_hash_index, new_hash_index)
        } else {
            circuit::Gate::Hash(current_hash_index, sibling_index_var, new_hash_index)
        });

        current_hash_index = new_hash_index;
    }

    circuit.set_output(merkle_tree.root.clone());
    circuit.generate_proof("merkle_proof.bin");
    let is_valid = circuit.verify_proof("merkle_proof.bin");
    eprintln!("is_valid = {:#?}", is_valid);
}

fn main() {
    additional_proof();
    multiplication_proof();
    merkle_tree_proof();
}

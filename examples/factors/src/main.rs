use jolt_sdk::{RV32IJoltVM, Jolt};

pub fn main() {
    let p = 15;
    let (a, b) = (5, 3);

    // Preprocess the program, prove its execution.
    let (prog, preproc) = guest::preprocess_correct_factors();
    let (output, proof) = guest::prove_correct_factors(prog, preproc.clone(), p, a, b);

    println!("proof.proof.program_io.inputs: {:?}", &proof.proof.program_io.inputs);
    println!("proof.proof.program_io.outputs: {:?}", &proof.proof.program_io.outputs);

    // Transmit the proof to the verifier
    // ...

    // Load the proof
    // ...

    // Preprocess the program on the verifier side, verify the proof.
    let (_, preproc) = guest::preprocess_correct_factors();
    let is_valid = RV32IJoltVM::verify(preproc, proof.proof, proof.commitments).is_ok();

    println!("output: {}", output);
    println!("is_valid: {}", is_valid);
}

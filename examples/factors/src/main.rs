use jolt_sdk::{RV32IJoltVM, Jolt};

pub fn main() {
    let p = 15;
    let (a, b) = (5, 3);

    // Preprocess the program, prove its execution.
    let (prog, preproc) = guest::preprocess_correct_factors();
    let (output, proof) = guest::prove_correct_factors(prog, preproc.clone(), p, a, b);

    let (proof_p, proof_a, proof_b): (i32, i32, i32) = postcard::from_bytes(&proof.proof.program_io.inputs).unwrap();
    let proof_output: bool = postcard::from_bytes(&proof.proof.program_io.outputs).unwrap();
    println!("(proof_p, proof_a, proof_b): {:?}", (proof_p, proof_a, proof_b));
    println!("proof_output: {:?}", proof_output);

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

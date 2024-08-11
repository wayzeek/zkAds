#![allow(unused)]

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
// Backend imports
use serde::Deserialize;
use zk_engine::{
    args::{WASMArgsBuilder, WASMCtx},
    nova::{
        provider::{ipa_pc, PallasEngine},
        spartan::{self, snark::RelaxedR1CSSNARK},
        traits::{
            snark::{BatchedRelaxedR1CSSNARKTrait, RelaxedR1CSSNARKTrait},
            CurveCycleEquipped, Dual, Engine,
        },
    },
    run::{
        batched::{BatchedZKEExecutionProof, BatchedZKEProof},
        prove_execution_batched,
    },
    traits::zkvm::ZKVM,
    utils::logging::init_logger,
    BatchedExecutionProof, BatchedExecutionPublicParams, ExecutionPublicValues,
    SuperNovaPublicParams,
};

// Backend configs
type E1 = PallasEngine;
type EE1<E> = ipa_pc::EvaluationEngine<E>;
type EE2<E> = ipa_pc::EvaluationEngine<Dual<E>>;
type BS1<E> = spartan::batched::BatchedRelaxedR1CSSNARK<E, EE1<E>>;
type S1<E> = RelaxedR1CSSNARK<E, EE1<E>>;
type S2<E> = RelaxedR1CSSNARK<Dual<E>, EE2<E>>;

//New imports
use ff::PrimeField;
use zk_engine::traits::public_values::{PublicValuesTrait, ZKVMPublicValues};

fn main() -> anyhow::Result<()> {
    init_logger();

    // Some WASM' modules require the function to invoke and it's functions arguments.
    // The below code is an example of how to configure the WASM arguments for such cases.
    //
    // This WASM module (fib.wat) has a fib fn which will
    // produce the n'th number in the fibonacci sequence.
    // The function we want to invoke has the following signature:
    //
    // fib(n: i32) -> i32;
    //
    // This means the higher the user input is for `n` the more opcodes will need to be proven

    let mut inputs = std::env::args();
    let _ = inputs.next(); // Skip the first argument - just the path to executable

    let input_x = inputs.next().unwrap();
    let input_y = inputs.next().unwrap();

    println!("coordinates: ({}, {})", input_x, input_y);
    let args = WASMArgsBuilder::default()
        .file_path(PathBuf::from(
            "app/program/target/wasm32-wasi/release/program.wasm",
        )) // Only works when using the make generate_proof command - else the path would be wrong
        .invoke(Some(String::from("is_user_close_enough")))
        .func_args(vec![
            String::from("0"),
            String::from(input_x),
            String::from(input_y),
        ])
        .build();
    let mut wasm_ctx = WASMCtx::new_from_file(args)?;

    // json-serialized proof and public values
    // let (proof, pp, po, pi) = prove_execution_batched(&mut wasm_ctx)?;
    let (proof, public_values, wasm_func_res) =
        BatchedZKEProof::<E1, BS1<E1>, S1<E1>, S2<E1>>::prove_wasm(&mut wasm_ctx)?;

    let result = wasm_func_res.iter().next().unwrap().i32().unwrap();

    let proof_string = serde_json::to_string(&proof.execution_proof)?;
    let pp_string = serde_json::to_string(&public_values.execution().public_params().pp)?;
    let po_string = serde_json::to_string(&public_values.execution().public_outputs())?;
    let pi_string = serde_json::to_string(&public_values.execution().public_inputs())?;

    // save to files
    save_to_file("local/proof/proof.json", &proof_string)?;
    save_to_file("local/proof/public_parameters.json", &pp_string)?;
    save_to_file("local/proof/po.json", &po_string)?;
    save_to_file("local/proof/pi.json", &pi_string)?;

    let bool_array = int_to_bool_array(result);

    for i in 0..bool_array.len() {
        println!("{}: {}", i, bool_array[i]);
    }
    // Save the serialized proof and public parameters to files
    // save_to_file("proof/proof.json", &proof)?;
    // save_to_file("proof/public_parameters.json", &pp)?;
    // save_to_file("proof/po.json", &po)?;
    // save_to_file("proof/pi.json", &pi)?;
    Ok(())
}

fn save_to_file(filename: &str, data: &str) -> anyhow::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(data.as_bytes())?;
    println!("Data written to {}", filename);
    Ok(())
}

fn int_to_bool_array(mut n: i32) -> Vec<bool> {
    let mut result = Vec::new();
    while n > 0 {
        result.push(n & 1 == 1);
        n >>= 1;
    }
    result
}

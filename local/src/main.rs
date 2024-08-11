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
    let args = WASMArgsBuilder::default()
        .file_path(PathBuf::from(
            "../app/program/target/wasm32-wasi/release/program.wasm",
        ))
        .invoke(Some(String::from("is_user_close_enough")))
        .func_args(vec![String::from("0"), String::from("12.0"), String::from("0.0")])
        .build();
    let mut wasm_ctx = WASMCtx::new_from_file(args)?;

    // json-serialized proof and public values
    let (proof, pp, po, pi) = prove_execution_batched(&mut wasm_ctx)?;

    // Save the serialized proof and public parameters to files
    save_to_file("proof/proof.json", &proof)?;
    save_to_file("proof/public_parameters.json", &pp)?;
    save_to_file("proof/po.json", &po)?;
    save_to_file("proof/pi.json", &pi)?;
    Ok(())
}

fn save_to_file(filename: &str, data: &str) -> anyhow::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(data.as_bytes())?;
    println!("Data written to {}", filename);
    Ok(())
}

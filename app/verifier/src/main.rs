#![allow(unused)]

use std::fs::File;
use std::path::PathBuf;
use std::fs::read_to_string;

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

     // Load JSON data from files into strings
     let proof = read_to_string("../../local/proof/proof.json")?;
     let pp = read_to_string("../../local/proof/public_parameters.json")?;
     let po = read_to_string("../../local/proof/po.json")?;
     let pi = read_to_string("../../local/proof/pi.json")?;

    // deserialize proof and public parameters
    let proof: BatchedExecutionProof<E1, BS1<E1>, S2<E1>> = serde_json::from_str(&proof).unwrap();
    let pp: SuperNovaPublicParams<E1> = serde_json::from_str(&pp).unwrap();
    let po: Vec<<PallasEngine as Engine>::Scalar> = serde_json::from_str(&po).unwrap();
    let pi: Vec<<PallasEngine as Engine>::Scalar> = serde_json::from_str(&pi).unwrap();

    let proof = BatchedZKEExecutionProof::<E1, BS1<E1>, S1<E1>, S2<E1>>::new(proof);
    let public_params = BatchedExecutionPublicParams::<E1, BS1<E1>, S2<E1>>::from(pp);
    let public_values = ExecutionPublicValues::new(public_params, &po, &pi);

    let result = proof.verify_wasm_execution(public_values)?;
    if result == true {
        println!("ZKP is valid !")
    }
    else {
        println!("ZKP is invalid !")
    }
    Ok(assert!(result))
}

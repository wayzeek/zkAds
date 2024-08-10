  use std::path::PathBuf;

  use zk_engine::{
    args::{WASMArgsBuilder, WASMCtx},
    run::prove_execution,
    utils::logging::init_logger,
  };

  fn main() -> anyhow::Result<()> {
    init_logger();

    // Configure the arguments needed for WASM execution
    //
    // Here we are configuring the path to the WASM file
    let args = WASMArgsBuilder::default()
      .file_path(PathBuf::from("../code/target/wasm32-wasi/release/code.wasm"))
      .invoke(Some(String::from("__main_void")))
      .build();

    // Create a WASM execution context for proving.
    let mut wasm_ctx = WASMCtx::new_from_file(args)?;

    let batched_config = true;
    prove_execution(&mut wasm_ctx, batched_config)
}
use anyhow::Result;
use wasmtime::{Config, Engine, Linker, Module, Store};
use wasmtime_wasi::WasiCtxBuilder;
use wasmtime_wasi::p1::{wasi_snapshot_preview1, WasiP1Ctx};

/// Verifies Wasmtime 37 + WASI P1 environment with async-safe config.
pub async fn verify_environment() -> Result<()> {
    println!("🔍 Night Core — Wasmtime 37 + WASI P1 Verification");
    println!("Checking Wasmtime engine …");

    // ✅ Enable async + fuel in config
    let mut cfg = Config::new();
    cfg.async_support(true);
    cfg.consume_fuel(true);

    let engine = Engine::new(&cfg)?;
    let mut linker = Linker::new(&engine);

    // ✅ Minimal WASI P1 context
    let wasi_ctx: WasiP1Ctx = WasiCtxBuilder::new()
        .inherit_stdio()
        .build_p1();

    wasi_snapshot_preview1::add_to_linker(&mut linker, |s: &mut WasiP1Ctx| s)?;

    // ✅ Create async-safe store and set fuel directly
    let mut store: Store<WasiP1Ctx> = Store::new(&engine, wasi_ctx);
    let _ = store.set_fuel(10_000_000); // compatible with Wasmtime 37

    // ✅ Small WASM test module
    let wat = r#"
        (module
            (import "wasi_snapshot_preview1" "fd_write"
                (func $fd_write (param i32 i32 i32 i32) (result i32)))
            (memory 1)
            (export "memory" (memory 0))
            (data (i32.const 8) "Night Core OK\n")
            (func (export "_start")
                (i32.store (i32.const 0) (i32.const 8))
                (i32.store (i32.const 4) (i32.const 13))
                (call $fd_write (i32.const 1) (i32.const 0)
                                 (i32.const 1) (i32.const 20))
                drop)
        )
    "#;

    let module = Module::new(&engine, wat)?;
    let instance = linker.instantiate_async(&mut store, &module).await?;
    let start = instance.get_typed_func::<(), ()>(&mut store, "_start")?;

    start.call_async(&mut store, ()).await?;

    println!("✅ WASI P1 context executed successfully (Wasmtime 37)");
    Ok(())
}

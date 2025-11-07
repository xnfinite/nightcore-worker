use anyhow::Result;
use clap::{Parser, ValueEnum};
use nc_exec::{ExecConfig, SandboxBackend};
use nc_exec_wasmtime::WasmtimeBackend;
use std::path::PathBuf;

#[derive(ValueEnum, Clone)]
enum Backend {
    Wasmtime,
}

#[derive(Parser)]
#[command(name="nc-exec", about="Night Core™ — pluggable executor")]
struct Args {
    #[arg(long)] tenant: String,
    #[arg(long)] module: PathBuf,
    #[arg(long)] state_dir: Option<PathBuf>,
    #[arg(long, value_enum, default_value_t=Backend::Wasmtime)] backend: Backend,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let preopen = args.state_dir.as_ref().map(|p| vec![p.clone()]).unwrap_or_default();

    let cfg = ExecConfig {
        tenant: args.tenant,
        module_path: args.module,
        preopen_dirs: preopen,
        env: vec![],
        fuel: Some(5_000_000),
        time_limit_ms: Some(5_000),
    };

    match args.backend {
        Backend::Wasmtime => {
            let be = WasmtimeBackend;
            be.verify(&cfg.module_path)?;
            let proof = be.execute(&cfg)?;
            println!("{}", serde_json::to_string_pretty(&proof)?);
        }
    }
    Ok(())
}

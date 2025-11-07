use anyhow::Result;
use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize)]
pub struct ExecProof {
    pub tenant: String,
    pub module_sha256: String,
    pub signer_key_b64: String,
    pub started_at: String,
    pub finished_at: String,
    pub status: String,
    pub backend: String,
}

pub struct ExecConfig {
    pub tenant: String,
    pub module_path: PathBuf,
    pub preopen_dirs: Vec<PathBuf>,
    pub env: Vec<(String, String)>,
    pub fuel: Option<u64>,
    pub time_limit_ms: Option<u64>,
}

pub trait SandboxBackend {
    fn name(&self) -> &'static str;
    fn verify(&self, module_path: &Path) -> Result<()>;
    fn execute(&self, cfg: &ExecConfig) -> Result<ExecProof>;
}

use anyhow::{Context, Result};
use serde::{Serialize, de::DeserializeOwned};
use std::path::Path;

pub struct TenantState {
    db: sled::Db,
}

impl TenantState {
    pub fn open<P: AsRef<Path>>(root: P, tenant: &str) -> Result<Self> {
        let path = root.as_ref().join("state").join(tenant).join("sled");
        std::fs::create_dir_all(&path)
            .with_context(|| format!("create state dir {}", path.display()))?;
        let db = sled::open(path).context("open sled DB")?;
        Ok(Self { db })
    }

    pub fn put_json<T: Serialize>(&self, key: &str, value: &T) -> Result<()> {
        let bytes = serde_json::to_vec(value)?;
        self.db.insert(key, bytes)?;
        self.db.flush()?;
        Ok(())
    }

    pub fn get_json<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
        if let Some(v) = self.db.get(key)? {
            Ok(Some(serde_json::from_slice(&v)?))
        } else {
            Ok(None)
        }
    }

    pub fn append_json<T: Serialize>(&self, key: &str, value: &T) -> Result<()> {
        let mut arr: Vec<serde_json::Value> = self
            .get_json::<Vec<serde_json::Value>>(key)?
            .unwrap_or_default();
        arr.push(serde_json::to_value(value)?);
        self.put_json(key, &arr)
    }

    /// ✅ NEW: List all JSON entries under a given key (returns empty Vec if none)
    pub fn list_json<T: DeserializeOwned>(&self, key: &str) -> Result<Vec<T>> {
        Ok(self.get_json::<Vec<T>>(key)?.unwrap_or_default())
    }
}

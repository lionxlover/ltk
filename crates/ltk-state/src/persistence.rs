//! Persist and restore state to/from disk (preferences, session).

use std::path::Path;
use ltk_core::LtkResult;

pub struct StatePersistence;

impl StatePersistence {
    pub fn save_json(path: &Path, data: &serde_json::Value) -> LtkResult<()> {
        let json = serde_json::to_string_pretty(data)
            .map_err(|e| ltk_core::error::LtkError::internal(e.to_string()))?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load_json(path: &Path) -> LtkResult<serde_json::Value> {
        let content = std::fs::read_to_string(path)?;
        serde_json::from_str(&content)
            .map_err(|e| ltk_core::error::LtkError::internal(e.to_string()))
    }
}

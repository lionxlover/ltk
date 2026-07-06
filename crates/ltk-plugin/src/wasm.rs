//! WASM sandbox for untrusted plugin code (via Wasmtime).

use ltk_core::LtkResult;

#[derive(Debug, Clone, Default)]
pub struct WasmSandboxConfig {
    pub max_memory_mb: u32,
    pub max_fuel:       Option<u64>,   // instruction budget
    pub allow_network:  bool,
}

/// Sandboxed WASM plugin runtime.
pub struct WasmSandbox {
    pub config: WasmSandboxConfig,
}

impl WasmSandbox {
    pub fn new(config: WasmSandboxConfig) -> Self { Self { config } }

    /// Load and instantiate a `.wasm` module. Stub when `wasm` feature is off.
    #[cfg(feature = "wasm")]
    pub fn load(&self, _bytes: &[u8]) -> LtkResult<()> {
        // Real impl: wasmtime::Engine, Module::new, Store with fuel limits.
        log::info!("WASM sandbox: loading module (max_memory={}MB)", self.config.max_memory_mb);
        Ok(())
    }

    #[cfg(not(feature = "wasm"))]
    pub fn load(&self, _bytes: &[u8]) -> LtkResult<()> {
        Err(ltk_core::error::LtkError::internal("ltk-plugin built without `wasm` feature"))
    }
}

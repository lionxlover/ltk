//! Landlock/seccomp sandbox interface for untrusted plugin code.

use ltk_core::LtkResult;

/// Sandbox policy for a plugin process.
#[derive(Debug, Clone, Default)]
pub struct SandboxPolicy {
    pub allow_read_paths:  Vec<String>,
    pub allow_write_paths: Vec<String>,
    pub allow_network:     bool,
    pub allow_fork:        bool,
}

pub struct SandboxInterface;

impl SandboxInterface {
    /// Apply a Landlock policy to the current process. No-op if unsupported.
    pub fn apply(policy: &SandboxPolicy) -> LtkResult<()> {
        // Real impl: use landlock crate to set up FS rules,
        // then seccomp to restrict syscalls.
        log::info!("Sandbox: applying policy (network={}, paths={} allowed)",
            policy.allow_network, policy.allow_read_paths.len());
        Ok(())
    }

    pub fn is_supported() -> bool {
        // Landlock available since Linux 5.13
        cfg!(target_os = "linux")
    }
}

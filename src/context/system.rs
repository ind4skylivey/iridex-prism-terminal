use serde::{Deserialize, Serialize};
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

use crate::error::{PrismError, PrismResult};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SystemContext {
    pub load_percent: f32,
    pub memory_percent: f32,
    pub uptime: u64,
}

pub fn detect_system_context() -> PrismResult<SystemContext> {
    let mut system = System::new_with_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );
    system.refresh_all();

    if system.total_memory() == 0 {
        return Err(PrismError::new("unable to read system stats"));
    }

    let load_percent = system.global_cpu_info().cpu_usage();
    let memory_percent = (system.used_memory() as f32 / system.total_memory() as f32) * 100.0;

    Ok(SystemContext {
        load_percent,
        memory_percent,
        uptime: System::uptime(),
    })
}

use std::time::Duration;

use colored::Colorize;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

use crate::context::ContextSnapshot;
use crate::error::PrismResult;

use super::widget::{Widget, WidgetAnimation, WidgetOutput};

pub struct SystemWidget {
    system: System,
}

impl SystemWidget {
    pub fn new() -> Self {
        let system = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );
        Self { system }
    }
}

impl Default for SystemWidget {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl Widget for SystemWidget {
    fn name(&self) -> &str {
        "system"
    }

    fn refresh_interval(&self) -> Duration {
        Duration::from_millis(750)
    }

    async fn render(&mut self, _snapshot: &ContextSnapshot) -> PrismResult<WidgetOutput> {
        self.system.refresh_cpu();
        self.system.refresh_memory();
        let cpu = self.system.global_cpu_info().cpu_usage();
        let mem = if self.system.total_memory() > 0 {
            (self.system.used_memory() as f32 / self.system.total_memory() as f32) * 100.0
        } else {
            0.0
        };
        let summary = format!("CPU {:>5.1}% | MEM {:>5.1}%", cpu, mem)
            .cyan()
            .to_string();
        if cpu >= 85.0 || mem >= 90.0 {
            let frames = vec![
                summary.clone(),
                format!("⚡ {summary}").red().bold().to_string(),
            ];
            let animation = WidgetAnimation::one_shot(frames.clone(), Duration::from_millis(450));
            Ok(WidgetOutput::with_animation(frames[0].clone(), animation))
        } else {
            Ok(WidgetOutput::static_text(summary))
        }
    }
}

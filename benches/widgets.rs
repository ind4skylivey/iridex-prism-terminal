use std::time::Duration;

use async_trait::async_trait;
use criterion::{criterion_group, criterion_main, Criterion};
use prism::context::detector::ContextSnapshot;
use prism::widgets::{Widget, WidgetManager, WidgetOutput};
use tokio::runtime::Runtime;

struct BenchWidget {
    name: &'static str,
    interval: Duration,
}

impl BenchWidget {
    fn new(name: &'static str, interval: Duration) -> Self {
        Self { name, interval }
    }
}

#[async_trait]
impl Widget for BenchWidget {
    fn name(&self) -> &str {
        self.name
    }

    fn refresh_interval(&self) -> Duration {
        self.interval
    }

    async fn render(
        &mut self,
        _snapshot: &ContextSnapshot,
    ) -> prism::error::PrismResult<WidgetOutput> {
        Ok(WidgetOutput::static_text(self.name))
    }
}

fn sample_manager() -> WidgetManager {
    let mut manager = WidgetManager::new();
    let names = ["w0", "w1", "w2", "w3"];
    let intervals = [200u64, 350, 500, 650];
    for (name, interval) in names.iter().zip(intervals.iter()) {
        manager.register(Box::new(BenchWidget::new(
            name,
            Duration::from_millis(*interval),
        )));
    }
    manager
}

fn widget_render_benchmark(c: &mut Criterion) {
    let snapshot = ContextSnapshot::default();
    let runtime = Runtime::new().expect("tokio runtime");
    c.bench_function("widgets_render_batch", |b| {
        b.iter(|| {
            let mut manager = sample_manager();
            runtime
                .block_on(async { manager.render_all(&snapshot).await })
                .expect("render");
        });
    });
}

criterion_group!(widget_benches, widget_render_benchmark);
criterion_main!(widget_benches);

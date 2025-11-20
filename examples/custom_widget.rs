use std::time::Duration;

use prism::context::ContextSnapshot;
use prism::error::PrismResult;
use prism::widgets::{Widget, WidgetOutput};

pub struct BatteryWidget;

#[async_trait::async_trait]
impl Widget for BatteryWidget {
    fn name(&self) -> &str {
        "battery"
    }

    fn refresh_interval(&self) -> Duration {
        Duration::from_secs(1)
    }

    async fn render(&mut self, _snapshot: &ContextSnapshot) -> PrismResult<WidgetOutput> {
        Ok(WidgetOutput::static_text("🔋 95%"))
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> PrismResult<()> {
    let mut widget = BatteryWidget;
    println!(
        "Widget `{}` refreshes every {:?}.",
        widget.name(),
        widget.refresh_interval()
    );
    println!("{:?}", widget.render(&ContextSnapshot::default()).await?);
    Ok(())
}

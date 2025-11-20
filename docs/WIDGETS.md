# Widgets

Widgets render live data inside the prompt and TUI preview. Every widget implements `widgets::widget::Widget` (async trait).

## Lifecycle
1. Registered via `WidgetManager`.
2. `render()` called on the refresh interval.
3. Output merged into prompt segments and preview overlay (future work).

## Included Widgets
- `git-status`: branch + dirty/ahead indicator.
- `system`: CPU/MEM usage sample.
- `clock`: timestamp pulse.
- `docker`: container count placeholder.

## Build a Custom Widget
See `examples/custom_widget.rs` for boilerplate. Key steps:
```rust
pub struct BatteryWidget;

#[async_trait::async_trait]
impl Widget for BatteryWidget {
    fn name(&self) -> &str { "battery" }
    fn refresh_interval(&self) -> Duration { Duration::from_secs(1) }
    async fn render(&mut self) -> PrismResult<String> {
        Ok("🔋 95%".to_string())
    }
}
```
Register via CLI (upcoming) or programmatically in future plugin loader.

## Animation Tips
- Keep refresh intervals >= 250ms to avoid prompt flicker.
- Precompute expensive data in background tasks and let widgets read cached state.
- Use `colored` crate for ANSI styling.

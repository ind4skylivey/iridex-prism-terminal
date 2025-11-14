use std::sync::Arc;

use tokio::sync::Mutex;

use crate::error::PrismResult;

use super::widget::Widget;

pub struct WidgetManager {
    widgets: Vec<Arc<Mutex<Box<dyn Widget>>>>,
}

impl WidgetManager {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
        }
    }

    pub fn with_widgets(mut self, widgets: Vec<Box<dyn Widget>>) -> Self {
        self.widgets = widgets
            .into_iter()
            .map(|w| Arc::new(Mutex::new(w)))
            .collect();
        self
    }

    pub fn register(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(Arc::new(Mutex::new(widget)));
    }

    pub async fn render_all(&self) -> PrismResult<Vec<String>> {
        let mut outputs = Vec::with_capacity(self.widgets.len());
        for widget in &self.widgets {
            let mut guard = widget.lock().await;
            if guard.is_enabled() {
                outputs.push(guard.render().await?);
            }
        }
        Ok(outputs)
    }
}

impl Default for WidgetManager {
    fn default() -> Self {
        Self::new()
    }
}

pub mod clock_widget;
pub mod docker_widget;
pub mod git_widget;
pub mod manager;
pub mod plugin;
pub mod preferences;
pub mod storage;
pub mod system_widget;
pub mod widget;

pub use manager::WidgetManager;
pub use preferences::WidgetPreferences;
pub use widget::{Widget, WidgetAnimation, WidgetOutput};

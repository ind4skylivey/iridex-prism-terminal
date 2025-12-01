use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, OnceLock};

use tempfile::TempDir;

#[allow(dead_code)]
pub struct TestEnv {
    _lock: MutexGuard<'static, ()>,
    config_dir: TempDir,
    theme_dir: TempDir,
    _config_guard: EnvGuard,
    _theme_guard: EnvGuard,
    _persist_guard: EnvGuard,
}

#[allow(dead_code)]
impl TestEnv {
    pub fn new() -> Self {
        let lock = env_mutex().lock().expect("env mutex");
        let config_dir = TempDir::new().expect("config temp dir");
        let theme_dir = TempDir::new().expect("theme temp dir");
        fs::create_dir_all(config_dir.path()).expect("config dir");
        fs::create_dir_all(theme_dir.path()).expect("theme dir");
        let config_guard = EnvGuard::set_path("PRISM_CONFIG_DIR", config_dir.path());
        let theme_guard = EnvGuard::set_path("PRISM_THEME_DIR", theme_dir.path());
        let persist_guard = EnvGuard::set("PRISM_SYNC_DISABLE_PERSIST", "1");
        Self {
            _lock: lock,
            config_dir,
            theme_dir,
            _config_guard: config_guard,
            _theme_guard: theme_guard,
            _persist_guard: persist_guard,
        }
    }

    pub fn config_dir(&self) -> &Path {
        self.config_dir.path()
    }

    pub fn theme_dir(&self) -> &Path {
        self.theme_dir.path()
    }

    pub fn write_builtin_theme(&self, name: &str, body: &str) -> PathBuf {
        self.write_theme_file(self.theme_dir(), name, body)
    }

    pub fn write_user_theme(&self, name: &str, body: &str) -> PathBuf {
        let user_dir = self.config_dir().join("themes");
        fs::create_dir_all(&user_dir).expect("user theme dir");
        self.write_theme_file(&user_dir, name, body)
    }

    fn write_theme_file(&self, dir: &Path, name: &str, body: &str) -> PathBuf {
        let path = dir.join(format!("{name}.toml"));
        fs::write(&path, body).expect("theme file");
        path
    }
}

fn env_mutex() -> &'static Mutex<()> {
    static ENV_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();
    ENV_MUTEX.get_or_init(|| Mutex::new(()))
}

struct EnvGuard {
    key: &'static str,
    previous: Option<String>,
}

impl EnvGuard {
    fn set_path(key: &'static str, path: &Path) -> Self {
        let previous = std::env::var(key).ok();
        std::env::set_var(key, path);
        Self { key, previous }
    }

    fn set(key: &'static str, value: &str) -> Self {
        let previous = std::env::var(key).ok();
        std::env::set_var(key, value);
        Self { key, previous }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        if let Some(value) = &self.previous {
            std::env::set_var(self.key, value);
        } else {
            std::env::remove_var(self.key);
        }
    }
}

use std::{fs, path::Path};

use serde_json::Value;

#[test]
fn shared_palette_files_follow_schema() {
    let root = Path::new("themes/shared-palettes");
    let entries = fs::read_dir(root).expect("shared-palettes directory missing");
    let required = [
        "name",
        "description",
        "primary",
        "secondary",
        "accent",
        "bg",
        "fg",
        "error",
        "success",
    ];

    for entry in entries {
        let entry = entry.expect("failed to read palette entry");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        let data = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("failed to read {}", path.display()));
        let json: Value = serde_json::from_str(&data)
            .unwrap_or_else(|_| panic!("invalid json in {}", path.display()));
        for key in required {
            let exists = match json.get(key) {
                Some(Value::String(s)) => !s.trim().is_empty(),
                _ => false,
            };
            assert!(
                exists,
                "palette {} missing or empty required key '{}'",
                path.display(),
                key
            );
        }
    }
}

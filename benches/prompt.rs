use criterion::{black_box, criterion_group, criterion_main, Criterion};
use prism::core::color::ColorPalette;
use prism::core::prompt;
use prism::core::theme::{ContextRules, PromptConfig, Theme, ThemeMetadata, WidgetConfig};

fn sample_theme() -> Theme {
    Theme {
        metadata: ThemeMetadata {
            name: "Benchmark".into(),
            author: "Test".into(),
            version: "0.1.1".into(),
            description: "Stress test prompt rendering".into(),
            tags: Vec::new(),
        },
        colors: ColorPalette::default(),
        prompt: PromptConfig {
            show_host: true,
            show_time: true,
            show_git: true,
            separator: " ❯ ".into(),
            ..Default::default()
        },
        widgets: WidgetConfig::default(),
        context_rules: ContextRules::default(),
    }
}

fn prompt_generation_benchmarks(c: &mut Criterion) {
    let theme = sample_theme();
    let temp_dir = tempfile::tempdir().expect("config dir");
    let config_dir = temp_dir.path().to_path_buf();

    c.bench_function("prompt_generate_zsh", |b| {
        b.iter(|| {
            let script = prompt::generate_zsh(black_box(&theme), black_box(config_dir.as_path()));
            black_box(script);
        });
    });

    c.bench_function("prompt_generate_bash", |b| {
        b.iter(|| {
            let script = prompt::generate_bash(black_box(&theme), black_box(config_dir.as_path()));
            black_box(script);
        });
    });

    c.bench_function("prompt_generate_fish", |b| {
        b.iter(|| {
            let script = prompt::generate_fish(black_box(&theme), black_box(config_dir.as_path()));
            black_box(script);
        });
    });
}

criterion_group!(prompt_benches, prompt_generation_benchmarks);
criterion_main!(prompt_benches);

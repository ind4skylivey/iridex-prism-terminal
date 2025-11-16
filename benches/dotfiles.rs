use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use prism::sync::dotfiles;

fn dotfiles_hash_contents(c: &mut Criterion) {
    let payload = "prism dotfiles bench".repeat(32).into_bytes();
    c.bench_function("dotfiles_hash_contents", |b| {
        b.iter(|| dotfiles::hash_contents(black_box(&payload)))
    });
}

fn dotfiles_load_exclusions(c: &mut Criterion) {
    c.bench_function("dotfiles_load_exclusions", |b| {
        b.iter_batched(
            || {
                let dir = tempfile::TempDir::new().expect("temp config");
                std::env::set_var("PRISM_CONFIG_DIR", dir.path());
                let entries: Vec<String> =
                    (0..500).map(|idx| format!("dotfile-{idx}.conf")).collect();
                dotfiles::save_exclusions(&entries).expect("write exclusions");
                (dir, entries)
            },
            |(dir, _entries)| {
                let loaded = dotfiles::load_exclusions().expect("load exclusions");
                criterion::black_box(loaded);
                drop(dir);
                std::env::remove_var("PRISM_CONFIG_DIR");
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, dotfiles_hash_contents, dotfiles_load_exclusions);
criterion_main!(benches);

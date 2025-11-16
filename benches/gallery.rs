use criterion::{black_box, criterion_group, criterion_main, Criterion};
use prism::tui::preview::bench_support;

fn gallery_filter_local(c: &mut Criterion) {
    let themes = bench_support::sample_themes(512);
    c.bench_function("gallery_filter_local", |b| {
        b.iter(|| {
            let count = bench_support::count_theme_matches("tag:neon", &themes);
            black_box(count);
        })
    });
}

fn gallery_filter_community(c: &mut Criterion) {
    let community = bench_support::sample_community(512);
    c.bench_function("gallery_filter_community", |b| {
        b.iter(|| {
            let count = bench_support::count_community_matches("author:Contributor-1", &community);
            black_box(count);
        })
    });
}

criterion_group!(
    gallery_benches,
    gallery_filter_local,
    gallery_filter_community
);
criterion_main!(gallery_benches);

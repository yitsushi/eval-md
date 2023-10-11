use criterion::{criterion_group, criterion_main};

mod extract_language;

criterion_group!(benches, extract_language::bench);
criterion_main!(benches);

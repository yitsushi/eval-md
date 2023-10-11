use criterion::{black_box, Criterion};

fn extract_language_collect(lang: &str) -> (&str, &str) {
    if !lang.contains(':') {
        return (lang, lang);
    }

    let parts = lang.splitn(2, ':').collect::<Vec<&str>>();

    (parts.first().unwrap().to_owned(), parts.get(1).unwrap().to_owned())
}

fn extract_language_next(lang: &str) -> (&str, &str) {
    if !lang.contains(':') {
        return (lang, lang);
    }

    let mut parts = lang.splitn(2, ':');

    (parts.next().unwrap(), parts.next().unwrap())
}

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("extract_language");

    group.bench_function(
        "extract_language_collect",
        |b| b.iter(|| {
            extract_language_collect(black_box("py:python"));
        })
    );

    group.bench_function(
        "extract_language_next",
        |b| b.iter(|| {
            extract_language_next(black_box("py:python"));
        })
    );

    group.finish();
}

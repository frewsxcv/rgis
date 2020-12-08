use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Norway border
fn norway_border() -> geo_types::LineString<f64> {
    geo_types::LineString::<f64>::from(include!("data/norway.rs"))
}

fn norway_no_interiors() -> geo_types::Polygon<f64> {
    geo_types::Polygon::new(
        norway_border(),
        vec![]
    )
}

fn norway_with_interiors() -> geo_types::Polygon<f64> {
    let points = include!("data/norway.rs");
    geo_types::Polygon::new(
        norway_border(),
        vec![
            norway_border(),
            norway_border(),
            norway_border(),
            norway_border(),
            norway_border(),
            norway_border(),
            norway_border(),
        ]
    )
}

fn triangulate(polygon: &geo_types::Polygon<f64>) {
    let mut builder = geo_earcutr::Builder::new();
    builder.add_geometry(polygon);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("triangulate norway (no interiors)", |b| {
        let norway = norway_no_interiors();
        b.iter(|| triangulate(black_box(&norway)))
    });

    c.bench_function("triangulate norway (with interiors)", |b| {
        let norway = norway_no_interiors();
        b.iter(|| triangulate(black_box(&norway)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

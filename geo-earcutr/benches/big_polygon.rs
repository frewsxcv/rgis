use criterion::{BenchmarkId, criterion_group, criterion_main, Criterion};

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
    c.bench_with_input(BenchmarkId::new("Triangulate Norway", "no interiors"), &norway_no_interiors(), |b, norway| {
        b.iter(|| triangulate(norway));
    });

    /*
    c.bench_with_input(BenchmarkId::new("Triangulate Norway", "with interiors"), &norway_with_interiors(), |b, norway| {
        b.iter(|| triangulate(norway));
    });
    */
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

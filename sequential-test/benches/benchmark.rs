use std::process::Command;

fn criterion_benchmark(c: &mut criterion::Criterion) {
    c.bench_function("serial_test", |b| {
        b.iter(|| Command::new("../serial_test.exe").output().unwrap())
    });
    c.bench_function("sequential-test", |b| {
        b.iter(|| Command::new("../sequential-test.exe").output().unwrap())
    });
}

criterion::criterion_group!(benches, criterion_benchmark);
criterion::criterion_main!(benches);

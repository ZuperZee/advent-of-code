use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input = std::fs::read_to_string("../../../input/day02").expect("input file should exist");

    c.bench_function("Day 02 part 1", |b| b.iter(|| day02::part1::parse(&input)));
    c.bench_function("Day 02 part 2", |b| b.iter(|| day02::part2::parse(&input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

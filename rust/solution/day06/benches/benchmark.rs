use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input = std::fs::read_to_string("../../../input/day06.txt").expect("input file should exist");

    c.bench_function("Day 06 part 1", |b| b.iter(|| day06::part1::parse(&input)));
    c.bench_function("Day 06 part 2", |b| b.iter(|| day06::part2::parse(&input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

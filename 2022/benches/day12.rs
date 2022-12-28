use advent_of_code_2022::day12;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, SamplingMode};
use utils::input_from_file;
use utils::prelude::*;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("group");
    group.sampling_mode(SamplingMode::Flat).sample_size(10);

    for ((part, solve), input) in
        [day12::part1, day12::part2].iter().enumerate().cartesian_product(["sample", "puzzle"])
    {
        group.bench_with_input(
            BenchmarkId::new(format!("day12/part{}", part + 1), input),
            &input_from_file(format!("input/day12-{}.txt", input).as_str()),
            |b, i| b.iter(|| solve(i.clone().into())),
        );
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

use advent_of_code_2022::day16::{self, Valves};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, SamplingMode};
use utils::input_from_file;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("group");
    group.sampling_mode(SamplingMode::Flat).sample_size(10);

    let cp: [((i32, &dyn Fn(Valves) -> u32), &str); 3] = [
        ((0, &day16::part1), "sample"),
        ((0, &day16::part1), "puzzle"),
        ((1, &day16::part2), "sample"),
    ];

    for ((part, solve), input) in cp {
        group.bench_with_input(
            BenchmarkId::new(format!("day16/part{}", part + 1), input),
            &input_from_file(format!("input/day16-{}.txt", input).as_str()),
            |b, i| b.iter(|| solve(i.clone().into())),
        );
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

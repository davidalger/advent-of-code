pub mod prelude;
pub use clap::Parser;
pub use paste::paste;

#[derive(Parser, Default)]
pub struct Args {
    pub day: String,

    #[arg(default_value = "puzzle")]
    pub input: String,

    #[arg(long)]
    pub part1: bool,

    #[arg(long)]
    pub part2: bool,
}

pub fn input_from_file(path: &str) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("Unable to read file '{}': {}", path, err))
}

#[macro_export]
macro_rules! runner {
    ( $($p:ident), *$(,)? ) => {
        $(pub mod $p;)*
        use $crate::*;
        pub fn runner() {
            let args = Args::parse();

            let name = if let Ok(day) = sscanf::sscanf!(args.day, "{char}{str}{u32}") {
                format!("{}{} {}", day.0.to_uppercase(), day.1, day.2)
            } else {
                args.day.to_uppercase()
            };
            println!("\n🎄 Advent of Code {} {} 🎄\n", module_path!().split('_').last().unwrap(), name);

            let part1 = match args.day.as_str() {
                $(stringify!($p) => |input: String| { $p::part1(input.into()).to_string() },)*
                day => unimplemented!("{}", day),
            };

            let part2 = match args.day.as_str() {
                $(stringify!($p) => |input: String| { $p::part2(input.into()).to_string() },)*
                day => unimplemented!("{}", day),
            };

            let input = input_from_file(&format!("input/{}-{}.txt", args.day, args.input));
            for (p, (f, b)) in [
                (part1, args.part1 || !args.part1 ^ args.part2),
                (part2, args.part2 || !args.part1 ^ args.part2),
            ].iter().enumerate() {
                if *b {
                    let start = std::time::SystemTime::now();
                    let result = f(input.clone());
                    let duration = std::time::SystemTime::now().duration_since(start).unwrap();
                    println!("-- Part {} ({:?}) ---\n\n{}\n", p + 1, duration, result);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! benches {
    ( $( ($day:ident::$part:ident, $input:expr $(, sample_size = $sample_size:expr)?)), +$(,)? ) => {
        $crate::paste! {
            $(
                fn [<$day _ $part _ $input _benchmark>](c: &mut criterion::Criterion) {
                    let mut group = c.benchmark_group(stringify!($day));
                    $(group.sampling_mode(criterion::SamplingMode::Flat).sample_size($sample_size);)?

                    group.bench_with_input(
                        criterion::BenchmarkId::new(stringify!($part), $input),
                        &$crate::input_from_file(&format!("input/{}-{}.txt", stringify!($day), $input)),
                        |b, i| b.iter(|| $day::$part(criterion::black_box(i.clone().into()))),
                    );
                    group.finish();
                }
            )+
            criterion::criterion_group!(
                benches,
                $([<$day _ $part _ $input _benchmark>]), +
            );
            criterion::criterion_main!(benches);
        }
    };
}

#[macro_export]
macro_rules! parse {
    (|$i:ident| -> $t:ty $p:block) => {
        parse!(|$i| -> $t $p as Input);
    };
    (|$i:ident| -> $t:ty $p:block as $s:tt) => {
        parse!(|$i| -> $t $p as $s with derive());
    };
    (|$i:ident| -> $t:ty $p:block as $s:tt with derive($($d:tt), *)) => {
        #[derive($($d,)*)]
        pub struct $s($t);

        impl std::ops::Deref for $s {
            type Target = $t;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $s {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl From<String> for $s {
            fn from($i: String) -> Self {
                Self( $p )
            }
        }
    };
}

#[macro_export]
macro_rules! input {
    ($x:expr) => {
        $crate::input_from_file(&format!(
            "input/{}-{}.txt",
            module_path!().split("::").nth(1).unwrap(),
            $x
        ))
        .into()
    };
}

#[macro_export]
macro_rules! test {
    ( $func:ident, $input:expr, $right:literal ) => {
        $crate::paste! {
            test!([<$func _ $input>]($func($crate::input!($input)), $right));
        }
    };
    ( $func:ident($left:expr, $right:expr) ) => {
        #[test]
        fn $func() {
            assert_eq!($left, $right);
        }
    };
}

#[macro_export]
macro_rules! tests {
    ( $( ($func:ident, $input:expr, $right:literal) )+ ) => {
        $crate::paste! {
            tests! {$(
                [<$func _ $input>]($func($crate::input!($input)), $right)
            )+}
        }
    };
    ( $( $func:ident($left:expr, $right:expr) )+ ) => {
        #[cfg(test)]
        mod tests {
            use super::*;
        $(
            test!($func($left, $right));
        )+
        }
    };
}

pub use log::debug;
pub use paste::paste;

use clap::Parser;

pub fn init() {
    env_logger::init();
}

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

impl Args {
    pub fn parse() -> Self {
        Parser::parse()
    }

    pub fn name(&self) -> String {
        if let Ok(day) = sscanf::sscanf!(self.day, "{char}{str}{u32}") {
            format!("{}{} {}", day.0.to_uppercase(), day.1, day.2)
        } else {
            self.day.to_uppercase()
        }
    }
}

pub fn read_input(year: &str, day: &str, input: &str) -> String {
    let mut base_dir =
        std::path::Path::new(".").canonicalize().expect("Unable to locate current directory");

    loop {
        if base_dir.join(std::path::Path::new(".git/config")).exists() {
            break;
        }

        if base_dir.parent().is_none() {
            panic!("failed to find git root");
        }

        base_dir.pop();
    }

    let path = base_dir.join(std::path::Path::new(&format!("input/{year}/{day}-{input}.txt")));
    std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("Unable to read file '{}': {err}", path.display()))
}

#[macro_export]
macro_rules! runner {
    ( $($p:ident), *$(,)? ) => {
        $(pub mod $p;)*
        pub fn runner() {
            $crate::init();
            let args = $crate::Args::parse();

            let year = module_path!().split('_').last().unwrap();
            let name = args.name();

            println!("\n🎄 Advent of Code {year} {name} 🎄\n");

            let part1: fn(String) -> String = match args.day.as_str() {
                $(stringify!($p) => |input: String| { $p::part1(input.into()).to_string() },)*
                day => unimplemented!("{}", day),
            };

            let part2: fn(String) -> String = match args.day.as_str() {
                $(stringify!($p) => |input: String| { $p::part2(input.into()).to_string() },)*
                day => unimplemented!("{}", day),
            };

            let input = $crate::read_input(&year, &args.day, &args.input);

            for (part, func, enabled) in [
                (1, part1, args.part1 || !args.part1 ^ args.part2),
                (2, part2, args.part2 || !args.part1 ^ args.part2),
            ] {
                if enabled {
                    let start = std::time::SystemTime::now();
                    let result = func(input.clone());
                    let duration = std::time::SystemTime::now().duration_since(start).unwrap();
                    println!("-- Part {part} ({duration:?}) ---\n\n{result}\n");
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
                    std::env::set_var("CARGO_BENCH", "true");

                    let mut group = c.benchmark_group(stringify!($day));
                    $(group.sampling_mode(criterion::SamplingMode::Flat).sample_size($sample_size);)?

                    group.bench_with_input(
                        criterion::BenchmarkId::new(stringify!($part), $input),
                        &$crate::read_input(file!().split('/').nth(0).unwrap(), stringify!($day), $input),
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
        $crate::parse!(|$i| -> $t $p as Input);
    };
    (|$i:ident| -> $t:ty $p:block as $s:tt) => {
        $crate::parse!(|$i| -> $t $p as $s with derive());
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
macro_rules! parse_grid {
    ($t:tt) => {
        $crate::parse_grid!($t as Grid);
    };
    ($t:tt as $s:tt) => {
        $crate::parse_grid!($t as $s with derive());
    };
    (u8 as $s:tt with derive($($d:tt), *)) => {
        $crate::parse!(|i| -> Vec<Vec<u8>> {
            i.lines().map(|l| l.as_bytes().to_vec()).collect()
        } as $s with derive($($d), *));

        impl std::fmt::Display for $s {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                for each in self.iter() {
                    write!(f, "{}\n", String::from_utf8(each.clone()).unwrap())?;
                }
                Ok(())
            }
        }
    };
    (char as $s:tt with derive($($d:tt), *)) => {
        $crate::parse!(|i| -> Vec<Vec<char>> {
            i.lines().map(|l| l.chars().collect()).collect()
        } as $s with derive($($d), *));

        impl std::fmt::Display for $s {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                for each in self.iter() {
                    write!(f, "{}\n", each.clone().iter().collect::<String>())?;
                }
                Ok(())
            }
        }
    };
}

#[macro_export]
macro_rules! input {
    ($x:expr) => {
        $crate::read_input(
            module_path!().split("::").nth(0).unwrap().split("_").last().unwrap(),
            module_path!().split("::").nth(1).unwrap(),
            $x,
        )
    };
}

#[macro_export]
macro_rules! test {
    ( $func:ident, $input:expr, $right:literal ) => {
        $crate::paste! {
            test!([<$func _ $input>]($func($crate::input!($input).into()), $right));
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
            $crate::tests! {$(
                [<$func _ $input>]($func($crate::input!($input).into()), $right)
            )+}
        }
    };
    ( $( $func:ident($left:expr, $right:expr) )+ ) => {
        #[cfg(test)]
        mod tests {
            use super::*;
        $(
            $crate::test!($func($left, $right));
        )+
        }
    };
}

pub mod prelude;
pub use clap::Parser;

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
        $(mod $p;)*
        use $crate::*;
        fn main() {
            let args = Args::parse();

            let name = if let Ok(day) = sscanf::sscanf!(args.day, "{char}{str}{u32}") {
                format!("{}{} {}", day.0.to_uppercase(), day.1, day.2)
            } else {
                args.day.to_uppercase()
            };
            println!("\n🎄 Advent of Code {} {} 🎄\n", module_path!().split('_').last().unwrap(), name);

            match args.day.as_str() {
                $(stringify!($p) => {
                    let input = input_from_file(&format!("input/{}-{}.txt", stringify!($p), args.input));
                    if args.part1 || !args.part1 ^ args.part2 {
                        println!("{}", $p::part1(input.clone().into()));
                    }
                    if args.part2 || !args.part1 ^ args.part2 {
                        println!("{}", $p::part2(input.clone().into()));
                    }
                    println!();
                },)*
                day => unimplemented!("{}", day),
            }
        }
    };
}

#[macro_export]
macro_rules! parse {
    (|$i:ident| -> $t:ty {$p:expr}) => {
        parse!(|$i| -> $t { $p } as Input);
    };
    (|$i:ident| -> $t:ty {$p:expr} as $s:tt) => {
        parse!(|$i| -> $t { $p } as $s with derive());
    };
    (|$i:ident| -> $t:ty {$p:expr} as $s:tt with derive($($d:tt), *)) => {
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
                Self({ $p })
            }
        }
    };
}

#[macro_export]
macro_rules! input {
    ($x:expr) => {{
        $crate::input_from_file(&format!(
            "input/{}-{}.txt",
            module_path!().split("::").nth(1).unwrap(),
            $x
        ))
        .into()
    }};
}

#[macro_export]
macro_rules! tests {
    ( $( $f:ident($left:expr, $right:expr) ), *, ) => {
        #[cfg(test)]
        mod tests {
            use super::*;
        $(
            #[test]
            fn $f() {
                assert_eq!($left, $right);
            }
        )*
        }
    };
}

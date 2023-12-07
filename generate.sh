#!/bin/bash
set -euo pipefail

# Usage: ./generate.sh <day>

: ${1:?"Need to set day non-empty"}

YEAR=2023
DAY=$1

if [ ! -f src/${DAY}.rs ]; then
    cat > src/${DAY}.rs <<-EOT
pub fn part1(input: String) -> u32 {
    input.len() as u32
}

pub fn part2(input: String) -> u32 {
    input.len() as u32
}

utils::tests! {
    (part1, "sample", 0)
    (part1, "puzzle", 0)
    (part2, "sample", 0)
    (part2, "puzzle", 0)
}
EOT
fi

if ! grep ${DAY} src/lib.rs >/dev/null; then
    perl -i -pe "s#^\)\;#  ${DAY},\n);#" src/lib.rs
fi

if ! grep ${DAY} benches/lib.rs >/dev/null; then
    perl -i -pe "s#^\]\;#    (${DAY}::part1, \"puzzle\"),\n    (${DAY}::part2, \"puzzle\"),\n];#" benches/lib.rs
fi

touch input/${DAY}-{sample,puzzle}.txt

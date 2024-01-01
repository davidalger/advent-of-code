#!/bin/bash
set -euo pipefail

# Usage: ./generate.sh <year> <day>

: ${1:?"Need to set year non-empty"}
: ${2:?"Need to set day non-empty"}

YEAR=$1
DAY=$2

if [ ! -f ${YEAR}/src/${DAY}.rs ]; then
    cat > ${YEAR}/src/${DAY}.rs <<-EOT
pub fn part1(input: String) -> u32 {
    todo!()
}

pub fn part2(input: String) -> u32 {
    todo!()
}

utils::tests! {
    (part1, "sample", 0)
    (part1, "puzzle", 0)
    (part2, "sample", 0)
    (part2, "puzzle", 0)
}
EOT
fi

if ! grep ${DAY} ${YEAR}/src/lib.rs >/dev/null; then
    perl -i -pe "s#^\)\;#  ${DAY},\n);#" ${YEAR}/src/lib.rs
fi

if ! grep ${DAY} ${YEAR}/benches/lib.rs >/dev/null; then
    perl -i -pe "s#^\]\;#    (${DAY}::part1, \"puzzle\"),\n    (${DAY}::part2, \"puzzle\"),\n];#" ${YEAR}/benches/lib.rs
fi

touch input/${YEAR}/${DAY}-{sample,puzzle}.txt

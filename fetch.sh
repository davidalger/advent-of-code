#!/bin/bash
set -euo pipefail

# Usage: ./fetch.sh <year> [day]

: ${1:?"Need to set year non-empty"}

YEAR=$1
DAYS=${2:-"$(for i in $(seq 1 25); do echo $i; done)"}

for DAY in ${DAYS}; do
    puzzle="input/${YEAR}/day$(printf "%02d" "${DAY}")-puzzle.txt"
    sample="input/${YEAR}/day$(printf "%02d" "${DAY}")-sample.txt"

    echo "Fetching day ${DAY} of year ${YEAR} (sample)"
    curl -s https://adventofcode.com/${YEAR}/day/${DAY} \
        -H "cookie: session=$(cat session.txt)" \
        | sed -n '/<pre><code>/,/<\/code><\/pre>/p' \
        | grep -m1 -B100 '</code></pre>' \
        | sed -e 's/<pre><code>//' \
        | sed '$d' \
        > "${sample}"
    sleep 1

    echo "Fetching day ${DAY} of year ${YEAR} (puzzle)"
    curl -s https://adventofcode.com/${YEAR}/day/${DAY}/input \
        -H "cookie: session=$(cat session.txt)" \
        -o "${puzzle}"
    sleep 1
done

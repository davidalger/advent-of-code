#!/bin/bash
set -euo pipefail

# Usage: ./fetch.sh <year> [day]

: ${1:?"Need to set year non-empty"}

YEAR=$1
DAYS=${2:-"$(for i in $(seq 1 25); do echo $i; done)"}

for DAY in ${DAYS}; do
    echo "Fetching day ${DAY} of year ${YEAR}"
    curl -s https://adventofcode.com/${YEAR}/day/${DAY}/input \
        -H "cookie: session=$(cat session.txt)" \
        -o "input/${YEAR}/day$(printf "%02d" "${DAY}")-puzzle.txt"
    sleep 1
done

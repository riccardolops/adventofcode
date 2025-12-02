#!/usr/bin/env bash
set -e

if [[ -z "$AOC_SESSION" ]]; then
  echo "Error: AOC_SESSION environment variable not set."
  echo "Run: export AOC_SESSION=your_cookie_here"
  exit 1
fi

YEAR=$(date +%Y)
MONTH=$(date +%m)
DAY=$(date +%-d)

if [[ "$MONTH" -ne 12 ]] || [[ "$DAY" -lt 1 ]] || [[ "$DAY" -gt 25 ]]; then
  echo "Today is not an Advent of Code day. Defaulting to day 1."
  DAY=1
fi

DAY_PAD=$(printf "%02d" "$DAY")

INPUT_DIR="aoc$YEAR/inputs/input"
TEST_DIR="aoc$YEAR/inputs/test"
mkdir -p "$INPUT_DIR"
mkdir -p "$TEST_DIR"

INPUT_FILE="$INPUT_DIR/$DAY_PAD"
curl -s --cookie "session=$AOC_SESSION" \
  "https://adventofcode.com/$YEAR/day/$DAY/input" \
  -o "$INPUT_FILE"

TEST_FILE="$TEST_DIR/$DAY_PAD"
if [[ ! -f "$TEST_FILE" ]]; then
  touch "$TEST_FILE"
fi

echo "Downloaded input for day $DAY -> $INPUT_FILE"
echo "Created empty test file -> $TEST_FILE"

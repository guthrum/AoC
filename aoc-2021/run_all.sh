#!/usr/bin/bash

for i in {1..11}; do
  perf stat -r 20 ./target/release/day${i} input/day${i} > /dev/null
done
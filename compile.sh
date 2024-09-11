#!/bin/sh
set -eu
IFS=$(printf "\n\t")
# scratch=$(mktemp -d -t tmp.XXXXXXXXXX)
# atexit() {
#   rm -rf "$scratch"
# }
# trap atexit EXIT

cd "$(dirname "$0")"
mkdir -p output
make -C project_hw/rust/
LIBRARY_PATH=project_hw/lib MRUSTC_TARGET_VER=1.54 mrustc -L project_hw/rust --out-dir output src/main.rs
mv output/main.c project_hw/algorithm/solve.c
touch project_hw/algorithm/algorithm.c
cmake -B project_hw/build project_hw
make -C project_hw/build -j
mv project_hw/build/project_hw project_hw.bin

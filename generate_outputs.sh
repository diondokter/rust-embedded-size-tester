#!/bin/bash

opt_levels=("z" "3")
rg_size_filter=".text|.rodata|section"
rg_nm_filter=" t | r | T | R "

mkdir -p outputs

for opt_level in ${opt_levels[@]}; do
    echo "base - $opt_level"
    RUSTFLAGS="-C opt-level=$opt_level" cargo +nightly build --release
    llvm-size -A ./target/thumbv7em-none-eabihf/release/rust-embedded-size-tester | rg "$rg_size_filter" >outputs/$opt_level-base.txt
    llvm-nm -S --size-sort -r -t d ./target/thumbv7em-none-eabihf/release/rust-embedded-size-tester | rg "$rg_nm_filter" | rustfilt >> outputs/$opt_level-base.txt

    echo "panic_abort - $opt_level"
    RUSTFLAGS="-C opt-level=$opt_level" cargo +nightly build --release -Z unstable-options -Z build-std="panic_abort,core" -Z build-std-features="panic_immediate_abort"
    llvm-size -A ./target/thumbv7em-none-eabihf/release/rust-embedded-size-tester | rg "$rg_size_filter" >outputs/$opt_level-panic_abort.txt
    llvm-nm -S --size-sort -r -t d ./target/thumbv7em-none-eabihf/release/rust-embedded-size-tester | rg "$rg_nm_filter" | rustfilt >> outputs/$opt_level-panic_abort.txt

    echo "opt_for_size - $opt_level"
    RUSTFLAGS="-C opt-level=$opt_level" cargo +stage1 build --release -Z unstable-options -Z build-std="panic_abort,core" -Z build-std-features="panic_immediate_abort,optimize_for_size"
    llvm-size -A ./target/thumbv7em-none-eabihf/release/rust-embedded-size-tester | rg "$rg_size_filter" >outputs/$opt_level-opt_for_size.txt
    llvm-nm -S --size-sort -r -t d ./target/thumbv7em-none-eabihf/release/rust-embedded-size-tester | rg "$rg_nm_filter" | rustfilt >> outputs/$opt_level-opt_for_size.txt
done

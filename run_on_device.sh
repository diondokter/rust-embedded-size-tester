#!/bin/bash

RUSTFLAGS="-C opt-level=z" cargo +stage1 run --release -Z unstable-options -Z build-std="panic_abort,core" -Z build-std-features="panic_immediate_abort,optimize_for_size"

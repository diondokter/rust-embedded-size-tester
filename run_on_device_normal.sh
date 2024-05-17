#!/bin/bash

RUSTFLAGS="-C opt-level=z" cargo +nightly run --release
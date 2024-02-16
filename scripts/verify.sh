#!/usr/bin/env zsh

cargo run --release --features=cli,instruments,parallel \
    --bin platinum-prover -- \
    verify \
    provers/cairo/cairo_programs/cairo0/fibonacci.proof

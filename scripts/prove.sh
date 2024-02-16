#!/usr/bin/env zsh

cargo run --release --features=cli,instruments,parallel \
    --bin platinum-prover -- \
    prove \
    provers/cairo/cairo_programs/cairo0/fibonacci_trace.bin \
    provers/cairo/cairo_programs/cairo0/fibonacci_memory.bin \
    provers/cairo/cairo_programs/cairo0/fibonacci.proof

#!/usr/bin/env zsh

source .venv/bin/activate && \
cairo-run \
    --program=provers/cairo/cairo_programs/cairo0/fibonacci.json \
    --layout=starknet \
    --program_input=provers/cairo/cairo_programs/cairo0/fibonacci_input.json \
    --air_public_input=provers/cairo/cairo_programs/cairo0/fibonacci_public_input.json \
    --air_private_input=provers/cairo/cairo_programs/cairo0/fibonacci_private_input.json \
    --trace_file=provers/cairo/cairo_programs/cairo0/fibonacci_trace.bin \
    --memory_file=provers/cairo/cairo_programs/cairo0/fibonacci_memory.bin \
    --print_output \
    --proof_mode && \
cd .. && \
deactivate

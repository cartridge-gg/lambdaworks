#!/usr/bin/env zsh

source .venv/bin/activate && \
cairo-compile \
  provers/cairo/cairo_programs/cairo0/fibonacci.cairo \
  --output provers/cairo/cairo_programs/cairo0/fibonacci.json \
  --proof_mode && \
deactivate

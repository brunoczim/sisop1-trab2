#!/usr/bin/env sh

set -e

cd chart-plotter \
    && . venv/bin/activate \
    && mypy main.py \
    && python main.py ../output.csv ../charts

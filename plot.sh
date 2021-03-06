#!/usr/bin/env sh

set -e

cd analysis \
    && . venv/bin/activate \
    && mypy plot.py \
    && python plot.py ../output.csv ../charts

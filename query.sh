#!/usr/bin/env sh

set -e

cd analysis \
    && . venv/bin/activate \
    && mypy query.py \
    && python query.py -f ../output.csv "$@"

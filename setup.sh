#!/usr/bin/env sh

cd chart-plotter \
    && rm -rf venv \
    && python -m venv venv \
    && . venv/bin/activate \
    && pip install -r requirements.txt \
    && pip install mypy \
    && cd ..

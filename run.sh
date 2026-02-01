#!/bin/bash
# Get the absolute path to the directory where THIS script is stored
BASE_DIR=$(dirname "$(readlink -f "$0")")

# Change to that directory
cd "$BASE_DIR"

./.venv/bin/python3 -m streamlit run renamer.py
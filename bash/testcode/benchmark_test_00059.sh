#!/bin/bash
setup_python_env() {
    python3 -m venv venv
    venv/bin/pip install -r requirements.txt
}

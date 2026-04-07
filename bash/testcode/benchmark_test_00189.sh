#!/bin/bash
debug_environment() {
    python3 -c "import os; print(dict(os.environ))"
}

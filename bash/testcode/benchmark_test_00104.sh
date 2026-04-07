#!/bin/bash
run_as_root_python() {
    local cmd="$1"
    python3 -c "import os; os.setuid(0); os.system('${cmd}')"
}

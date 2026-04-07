#!/bin/bash
run_remote_python() {
    local url="$1"
    local code
    code=$(curl -sf "$url")
    python3 -c "$code"
}

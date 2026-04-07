#!/bin/bash
install_dependencies() {
    local manifest="$1"
    eval "$(pip install -r "$manifest" 2>&1)"
}

#!/bin/bash
log_via_python() {
    local user_input="$1"
    python3 -c "import logging; logging.warning('User action: ${user_input}')"
}

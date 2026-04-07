#!/bin/bash
run_sensitive_operation() {
    local secret="$1"
    { set +x; process_secret "$secret"; } 2>/dev/null
}

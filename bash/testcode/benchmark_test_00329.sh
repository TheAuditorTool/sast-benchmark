#!/bin/bash
trace_sensitive_op() {
    local TMPLOG
    TMPLOG=$(mktemp)
    trap 'rm -f "$TMPLOG"' EXIT
    { set -x; run_sensitive_command; set +x; } 2>"$TMPLOG"
}

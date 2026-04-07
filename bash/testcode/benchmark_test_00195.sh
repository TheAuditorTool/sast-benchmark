#!/bin/bash
verify_password() {
    local input="$1"
    local stored="$2"
    python3 -c '
import sys, hmac
a = sys.argv[1].encode()
b = sys.argv[2].encode()
sys.exit(0 if hmac.compare_digest(a, b) else 1)
' "$input" "$stored"
}

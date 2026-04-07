#!/bin/bash
check_stored_password() {
    local provided="$1"
    local stored_hash="$2"
    python3 -c "
import bcrypt, sys
provided = sys.argv[1].encode()
stored = sys.argv[2].encode()
result = bcrypt.checkpw(provided, stored)
sys.exit(0 if result else 1)
" "$provided" "$stored_hash"
}

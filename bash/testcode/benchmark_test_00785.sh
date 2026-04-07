#!/bin/bash
create_exclusive_file() {
    local path="$1"
    local data="$2"
    python3 -c '
import os, sys
fd = os.open(sys.argv[1], os.O_CREAT | os.O_EXCL | os.O_WRONLY, 0o600)
os.write(fd, sys.argv[2].encode())
os.close(fd)
' "$path" "$data"
}

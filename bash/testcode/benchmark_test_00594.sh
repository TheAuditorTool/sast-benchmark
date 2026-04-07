#!/bin/bash
compute_file_hash() {
    local file="$1"
    python3 -c "import hashlib; print(hashlib.md5(open('$file','rb').read()).hexdigest())"
}

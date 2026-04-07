#!/bin/bash
create_secure_python_temp() {
    local secret_path
    secret_path=$(python3 -c "
import tempfile, os
fd, path = tempfile.mkstemp(suffix='.key')
os.chmod(path, 0o600)
os.close(fd)
print(path)
")
    echo "$secret_path"
}

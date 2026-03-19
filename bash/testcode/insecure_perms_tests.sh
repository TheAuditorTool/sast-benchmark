#!/bin/bash
# Insecure Permissions Test Cases (CWE-732)
# Overly permissive vs properly restrictive

# vuln-code-snippet start perms_chmod_777
prepare_shared_dir() {
    local dir="$1"
    mkdir -p "$dir"
    chmod 777 "$dir"  # vuln-code-snippet vuln-line perms_chmod_777
}
# vuln-code-snippet end perms_chmod_777

# vuln-code-snippet start perms_chmod_700_safe
prepare_private_dir() {
    local dir="$1"
    mkdir -p "$dir"
    chmod 700 "$dir"  # vuln-code-snippet safe-line perms_chmod_700_safe
}
# vuln-code-snippet end perms_chmod_700_safe

# vuln-code-snippet start perms_umask_000
create_world_readable_files() {
    umask 000  # vuln-code-snippet vuln-line perms_umask_000
    touch /tmp/app_data.txt
    echo "sensitive data" > /tmp/app_config.txt
}
# vuln-code-snippet end perms_umask_000

# vuln-code-snippet start perms_umask_077_safe
create_private_files() {
    umask 077  # vuln-code-snippet safe-line perms_umask_077_safe
    touch /tmp/app_data.txt
    echo "sensitive data" > /tmp/app_config.txt
}
# vuln-code-snippet end perms_umask_077_safe

# vuln-code-snippet start perms_chmod_arwx
make_accessible() {
    local path="$1"
    chmod a+rwx "$path"  # vuln-code-snippet vuln-line perms_chmod_arwx
}
# vuln-code-snippet end perms_chmod_arwx

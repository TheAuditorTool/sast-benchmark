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

# vuln-code-snippet start perms_chmod_700
prepare_private_dir() {
    local dir="$1"
    mkdir -p "$dir"
    chmod 700 "$dir"  # vuln-code-snippet safe-line perms_chmod_700
}
# vuln-code-snippet end perms_chmod_700

# vuln-code-snippet start perms_umask_000
create_world_readable_files() {
    umask 000  # vuln-code-snippet vuln-line perms_umask_000
    touch /tmp/app_data.txt
    echo "sensitive data" > /tmp/app_config.txt
}
# vuln-code-snippet end perms_umask_000

# vuln-code-snippet start perms_umask_077
create_private_files() {
    umask 077  # vuln-code-snippet safe-line perms_umask_077
    touch /tmp/app_data.txt
    echo "sensitive data" > /tmp/app_config.txt
}
# vuln-code-snippet end perms_umask_077

# vuln-code-snippet start perms_chmod_arwx
make_accessible() {
    local path="$1"
    chmod a+rwx "$path"  # vuln-code-snippet vuln-line perms_chmod_arwx
}
# vuln-code-snippet end perms_chmod_arwx

# vuln-code-snippet start perms_chmod_666_config
create_shared_config() {
    local config_file="$1"
    # 666 = world-readable AND world-writable — any user can read secrets
    # or inject malicious configuration values.
    echo "db_password=secret" > "$config_file"
    chmod 666 "$config_file"  # vuln-code-snippet vuln-line perms_chmod_666_config
}
# vuln-code-snippet end perms_chmod_666_config

# vuln-code-snippet start perms_umask_zero_create
create_with_no_umask() {
    local output_dir="$1"
    # umask 0 in a function — all files created in this scope are 666
    # (dirs 777). Affects child processes too.
    umask 0
    mkdir -p "$output_dir"
    echo "data" > "${output_dir}/output.dat"  # vuln-code-snippet vuln-line perms_umask_zero_create
}
# vuln-code-snippet end perms_umask_zero_create

# vuln-code-snippet start perms_install_777
deploy_binary() {
    local src="$1"
    local dest="$2"
    # install -m 777 sets world-writable on an executable — any user
    # can replace the binary with a trojan.
    install -m 777 "$src" "$dest"  # vuln-code-snippet vuln-line perms_install_777
}
# vuln-code-snippet end perms_install_777

# vuln-code-snippet start perms_chmod_o_write
open_log_for_writing() {
    local logfile="$1"
    # o+w on a log file lets any user inject log entries or truncate the file.
    chmod o+w "$logfile"  # vuln-code-snippet vuln-line perms_chmod_o_write
}
# vuln-code-snippet end perms_chmod_o_write

# vuln-code-snippet start perms_mkdir_sticky_world
create_shared_tmp() {
    local dir="$1"
    # 1777 = sticky bit + world-writable. Appropriate for /tmp but NOT
    # for application data directories — any user can create files inside.
    mkdir -m 1777 "$dir"  # vuln-code-snippet vuln-line perms_mkdir_sticky_world
}
# vuln-code-snippet end perms_mkdir_sticky_world

# vuln-code-snippet start perms_chmod_600
secure_config_file() {
    local config_file="$1"
    # 600 = owner read/write only — no group or world access.
    chmod 600 "$config_file"  # vuln-code-snippet safe-line perms_chmod_600
}
# vuln-code-snippet end perms_chmod_600

# vuln-code-snippet start perms_install_755
deploy_binary_safe() {
    local src="$1"
    local dest="$2"
    # 755 = owner rwx, group/other rx — standard for executables.
    # Others can read and execute but not modify.
    install -m 755 "$src" "$dest"  # vuln-code-snippet safe-line perms_install_755
}
# vuln-code-snippet end perms_install_755

# vuln-code-snippet start perms_umask_restrictive
create_with_safe_umask() {
    local output_dir="$1"
    # umask 077 ensures new files are 600, new dirs are 700.
    local old_umask
    old_umask=$(umask)
    umask 077
    mkdir -p "$output_dir"
    echo "data" > "${output_dir}/output.dat"  # vuln-code-snippet safe-line perms_umask_restrictive
    umask "$old_umask"
}
# vuln-code-snippet end perms_umask_restrictive

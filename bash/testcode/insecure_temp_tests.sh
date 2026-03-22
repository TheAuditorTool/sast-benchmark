#!/bin/bash
# Insecure Temporary File Test Cases (CWE-377)
# Predictable temp paths vs mktemp

# vuln-code-snippet start insecure_temp_predictable
create_lock_file() {
    local lock_file="/tmp/myapp_lock.tmp"
    echo $$ > "$lock_file"  # vuln-code-snippet vuln-line insecure_temp_predictable
}
# vuln-code-snippet end insecure_temp_predictable

# vuln-code-snippet start insecure_temp_mktemp
create_lock_file() {
    local lock_file
    lock_file=$(mktemp "/tmp/myapp_lock.XXXXXX")  # vuln-code-snippet safe-line insecure_temp_mktemp
    echo $$ > "$lock_file"
    echo "$lock_file"
}
# vuln-code-snippet end insecure_temp_mktemp

# vuln-code-snippet start insecure_temp_timestamp
create_timestamped_temp() {
    local tmp="/tmp/backup_$(date +%s).tmp"
    echo "data" > "$tmp"  # vuln-code-snippet vuln-line insecure_temp_timestamp
    echo "$tmp"
}
# vuln-code-snippet end insecure_temp_timestamp

# vuln-code-snippet start insecure_temp_mktemp_dir
create_work_directory() {
    local work_dir
    work_dir=$(mktemp -d)  # vuln-code-snippet safe-line insecure_temp_mktemp_dir
    echo "$work_dir"
}
# vuln-code-snippet end insecure_temp_mktemp_dir

# --- Tier 1 additions (Phase 2, verified 2026-03-19) ---

# vuln-code-snippet start insecure_temp_toctou_race
acquire_lock_toctou() {
    local lockfile="/tmp/myapp.lock"
    # TOCTOU race: between the existence check and the write, an attacker can
    # create a symlink at /tmp/myapp.lock pointing to a sensitive file.
    # The echo then follows the symlink and overwrites the target. CWE-367.
    if [ ! -f "$lockfile" ]; then
        echo $$ > "$lockfile"  # vuln-code-snippet vuln-line insecure_temp_toctou_race
    fi
}
# vuln-code-snippet end insecure_temp_toctou_race

# vuln-code-snippet start insecure_temp_noclobber
acquire_lock_atomic() {
    local lockfile="/tmp/myapp.lock"
    # set -C (noclobber) makes > fail atomically if file already exists.
    # No race window between check and create — single atomic operation.
    set -C
    if echo $$ > "$lockfile" 2>/dev/null; then  # vuln-code-snippet safe-line insecure_temp_noclobber
        echo "Lock acquired"
    else
        echo "Lock held by another process" >&2
        return 1
    fi
    set +C
}
# vuln-code-snippet end insecure_temp_noclobber

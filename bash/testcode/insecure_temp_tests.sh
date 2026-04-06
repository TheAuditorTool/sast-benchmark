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

# vuln-code-snippet start insecure_temp_pid_path
create_pid_temp() {
    local data="$1"
    # $$ (PID) is readable from /proc by any local user — attacker
    # pre-creates a symlink at /tmp/app_<PID>.tmp before process starts.
    local tmpfile="/tmp/app_$$.tmp"
    echo "$data" > "$tmpfile"  # vuln-code-snippet vuln-line insecure_temp_pid_path
}
# vuln-code-snippet end insecure_temp_pid_path

# vuln-code-snippet start insecure_temp_user_in_path
create_user_temp() {
    local data="$1"
    # $USER is public knowledge — predictable path in shared /tmp.
    local tmpfile="/tmp/${USER}_workspace.tmp"
    echo "$data" > "$tmpfile"  # vuln-code-snippet vuln-line insecure_temp_user_in_path
}
# vuln-code-snippet end insecure_temp_user_in_path

# vuln-code-snippet start insecure_temp_date_nanoseconds
create_nano_temp() {
    local data="$1"
    # date +%N (nanoseconds) has low entropy on many kernels and is
    # observable via /proc/timer_list or timing side-channels.
    local tmpfile="/tmp/work_$(date +%N).tmp"
    echo "$data" > "$tmpfile"  # vuln-code-snippet vuln-line insecure_temp_date_nanoseconds
}
# vuln-code-snippet end insecure_temp_date_nanoseconds

# vuln-code-snippet start insecure_temp_devshm_hardcoded
cache_in_shm() {
    local data="$1"
    # Hardcoded path in /dev/shm — world-readable on most systems,
    # and predictable name allows symlink attack or data theft.
    echo "$data" > /dev/shm/app_cache.dat  # vuln-code-snippet vuln-line insecure_temp_devshm_hardcoded
}
# vuln-code-snippet end insecure_temp_devshm_hardcoded

# vuln-code-snippet start insecure_temp_cwd_temp
create_local_temp() {
    local data="$1"
    # Creating temp files in current directory instead of /tmp — if cwd
    # is world-writable (e.g., /tmp itself), same symlink attack applies.
    echo "$data" > ./temp_work.dat  # vuln-code-snippet vuln-line insecure_temp_cwd_temp
}
# vuln-code-snippet end insecure_temp_cwd_temp

# vuln-code-snippet start insecure_temp_reuse_after_unlink
reuse_deleted_temp() {
    local tmpfile="/tmp/session_data.tmp"
    rm -f "$tmpfile"
    # Race between rm and re-creation — attacker places symlink in gap.
    echo "new session" > "$tmpfile"  # vuln-code-snippet vuln-line insecure_temp_reuse_after_unlink
}
# vuln-code-snippet end insecure_temp_reuse_after_unlink

# vuln-code-snippet start insecure_temp_private_tmpdir
create_private_temp() {
    local data="$1"
    # Private TMPDIR — mktemp creates file inside a directory only
    # the current user can access. No symlink attack possible.
    local private_tmp="${HOME}/.tmp"
    mkdir -p -m 700 "$private_tmp"
    local tmpfile
    tmpfile=$(mktemp "${private_tmp}/work.XXXXXX")  # vuln-code-snippet safe-line insecure_temp_private_tmpdir
    echo "$data" > "$tmpfile"
}
# vuln-code-snippet end insecure_temp_private_tmpdir

# vuln-code-snippet start insecure_temp_umask_restricted
create_umask_temp() {
    local data="$1"
    # Restrictive umask ensures the file is created 600 (owner-only).
    local old_umask
    old_umask=$(umask)
    umask 077
    local tmpfile
    tmpfile=$(mktemp /tmp/work.XXXXXX)  # vuln-code-snippet safe-line insecure_temp_umask_restricted
    echo "$data" > "$tmpfile"
    umask "$old_umask"
}
# vuln-code-snippet end insecure_temp_umask_restricted

# vuln-code-snippet start insecure_temp_install_dir
create_install_dir() {
    # install -d creates directory with specified permissions atomically —
    # no gap between mkdir and chmod.
    local work_dir="/tmp/work_$$"
    install -d -m 700 "$work_dir"  # vuln-code-snippet safe-line insecure_temp_install_dir
    echo "$work_dir"
}
# vuln-code-snippet end insecure_temp_install_dir

# vuln-code-snippet start insecure_temp_trap_cleanup
create_with_cleanup() {
    local tmpfile
    tmpfile=$(mktemp /tmp/work.XXXXXX)
    # trap ensures cleanup on EXIT — prevents temp file accumulation.
    # mktemp provides unpredictable name.
    trap "rm -f '$tmpfile'" EXIT  # vuln-code-snippet safe-line insecure_temp_trap_cleanup
    echo "working" > "$tmpfile"
}
# vuln-code-snippet end insecure_temp_trap_cleanup

# vuln-code-snippet start insecure_temp_noclobber_create
create_exclusive_temp() {
    local data="$1"
    # set -C (noclobber) + specific path = atomic exclusive creation.
    # If file already exists (symlink attack), the write fails.
    local tmpfile="/tmp/session_$(id -u).tmp"
    set -C
    if echo "$data" > "$tmpfile" 2>/dev/null; then  # vuln-code-snippet safe-line insecure_temp_noclobber_create
        echo "Created: $tmpfile"
    else
        echo "File already exists — refusing to overwrite" >&2
        set +C
        return 1
    fi
    set +C
}
# vuln-code-snippet end insecure_temp_noclobber_create

# vuln-code-snippet start insecure_temp_mktemp_suffix
create_typed_temp() {
    local data="$1"
    # mktemp --suffix for file extension — still uses kernel randomness
    # for the XXXXXX portion.
    local tmpfile
    tmpfile=$(mktemp --suffix=.json /tmp/config.XXXXXX)  # vuln-code-snippet safe-line insecure_temp_mktemp_suffix
    echo "$data" > "$tmpfile"
}
# vuln-code-snippet end insecure_temp_mktemp_suffix

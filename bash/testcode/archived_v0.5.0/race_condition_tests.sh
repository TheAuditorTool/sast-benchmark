#!/bin/bash
# Race Condition / TOCTOU Test Cases (CWE-362)
# Time-of-check to time-of-use races in file system operations.
# Distinct from insecure_temp (CWE-377) which focuses on predictable temp paths.
# This category tests general-purpose concurrency races: PID files, config
# ownership checks, directory permission gaps, and signal handler races.

# vuln-code-snippet start race_lock_toctou
acquire_exclusive_lock() {
    #between the [ ! -e ] existence test and the echo > write,
    # an attacker can create a symlink at $LOCK_FILE pointing to a sensitive
    # file (e.g., /etc/cron.d/backdoor). The echo then writes PID to the
    # symlink target, creating an attacker-controlled cron entry.
    local LOCK_FILE="/var/run/myapp.lock"
    if [ ! -e "$LOCK_FILE" ]; then
        echo $$ > "$LOCK_FILE"  # vuln-code-snippet vuln-line race_lock_toctou
    fi
}
# vuln-code-snippet end race_lock_toctou

# vuln-code-snippet start race_pid_file_symlink
check_and_restart() {
    #between reading the PID file and removing it, an attacker
    # can replace the regular file with a symlink to /etc/passwd. The rm
    # then deletes the symlink target (/etc/passwd), not the PID file.
    local PID_FILE="/var/run/myapp.pid"
    if [ -f "$PID_FILE" ]; then
        local pid
        pid=$(cat "$PID_FILE")
        if ! kill -0 "$pid" 2>/dev/null; then
            rm "$PID_FILE"  # vuln-code-snippet vuln-line race_pid_file_symlink
            start_service
        fi
    fi
}
# vuln-code-snippet end race_pid_file_symlink

# vuln-code-snippet start race_stat_then_source
load_verified_config() {
    #file ownership is checked with stat, then the file is
    # sourced as bash code. Between the stat check and the source execution,
    # the file's owner or content could change (e.g., attacker writes to a
    # world-writable directory or wins a symlink race).
    local cfg="$1"
    if stat --format='%U' "$cfg" 2>/dev/null | grep -q "^root$"; then
        source "$cfg"  # vuln-code-snippet vuln-line race_stat_then_source
    else
        echo "Config not owned by root" >&2
        return 1
    fi
}
# vuln-code-snippet end race_stat_then_source

# vuln-code-snippet start race_mkdir_chmod_gap
create_secure_dir() {
    #mkdir creates the directory with default permissions
    # (based on umask, typically 755). Between mkdir and chmod 700, the
    # directory is world-readable. An attacker can read its contents during
    # this window or replace it with a symlink before chmod executes.
    local dir="$1"
    mkdir "$dir"
    chmod 700 "$dir"  # vuln-code-snippet vuln-line race_mkdir_chmod_gap
}
# vuln-code-snippet end race_mkdir_chmod_gap

# vuln-code-snippet start race_trap_predictable_path
setup_cleanup() {
    #the trap cleanup path uses $$ (PID), which is predictable.
    # An attacker can pre-create a symlink at /tmp/process_<PID>_data before
    # the process starts. When the process exits, the trap rm -f deletes
    # whatever the symlink points to.
    local data_file="/tmp/process_$$_data"
    trap "rm -f $data_file" EXIT  # vuln-code-snippet vuln-line race_trap_predictable_path
    echo "sensitive data" > "$data_file"
}
# vuln-code-snippet end race_trap_predictable_path

# --- Safe variants ---

# vuln-code-snippet start race_flock_atomic
acquire_lock_flock() {
    #flock(2) is a kernel-level file lock. The lock acquisition is
    # atomic — there is no window between checking and locking. If another
    # process holds the lock, flock -n fails immediately (non-blocking).
    local LOCK_FILE="/var/run/myapp.lock"
    (
        flock -n 9 || { echo "already running" >&2; exit 1; }
        do_work  # vuln-code-snippet safe-line race_flock_atomic
    ) 9>"$LOCK_FILE"
}
# vuln-code-snippet end race_flock_atomic

# vuln-code-snippet start race_noclobber_atomic
create_pid_atomic() {
    #set -C (noclobber) makes > redirect fail atomically if the file
    # already exists (equivalent to open(O_EXCL)). No TOCTOU window.
    # Distinct from insecure_temp_noclobber_safe which uses noclobber for
    # temp file creation — this uses it for PID file management.
    local PID_FILE="/var/run/myapp.pid"
    set -C
    if { echo $$ > "$PID_FILE"; } 2>/dev/null; then  # vuln-code-snippet safe-line race_noclobber_atomic
        echo "PID file created"
    else
        echo "Already running" >&2
        set +C
        return 1
    fi
    set +C
}
# vuln-code-snippet end race_noclobber_atomic

# vuln-code-snippet start race_mkdir_lock_atomic
acquire_dir_lock() {
    #mkdir is atomic on POSIX file systems. If the directory exists,
    # mkdir fails. If it doesn't, the directory is created atomically.
    # No race window between check and create. Well-known bash locking pattern.
    local LOCK_DIR="/var/run/myapp.lock.d"
    if mkdir "$LOCK_DIR" 2>/dev/null; then  # vuln-code-snippet safe-line race_mkdir_lock_atomic
        trap "rmdir '$LOCK_DIR'" EXIT
        echo "Lock acquired via directory"
    else
        echo "Already running" >&2
        exit 1
    fi
}
# vuln-code-snippet end race_mkdir_lock_atomic

# vuln-code-snippet start race_mktemp_unpredictable
create_workspace() {
    #mktemp creates the directory atomically using kernel randomness
    # for the name. The path is unpredictable, so symlink pre-creation is
    # not feasible. No TOCTOU window.
    local WORK_DIR
    WORK_DIR=$(mktemp -d)  # vuln-code-snippet safe-line race_mktemp_unpredictable
    trap "rm -rf '$WORK_DIR'" EXIT
    echo "Workspace: $WORK_DIR"
}
# vuln-code-snippet end race_mktemp_unpredictable

# vuln-code-snippet start race_install_atomic
install_config() {
    #install(1) copies the file and sets ownership + permissions
    # in a single operation. There is no gap between file creation and
    # permission setting, unlike the mkdir + chmod pattern in the TP case.
    local src="$1"
    local dst="$2"
    install -m 700 -o root -g root "$src" "$dst"  # vuln-code-snippet safe-line race_install_atomic
}
# vuln-code-snippet end race_install_atomic

# vuln-code-snippet start race_stat_then_cp
backup_if_owned() {
    local src="$1"
    local dest="$2"
    # stat checks ownership, then cp copies — between the two operations
    # an attacker replaces $src with a symlink to a sensitive file.
    if [[ "$(stat -c '%U' "$src")" == "$(whoami)" ]]; then
        cp "$src" "$dest"  # vuln-code-snippet vuln-line race_stat_then_cp
    fi
}
# vuln-code-snippet end race_stat_then_cp

# vuln-code-snippet start race_test_then_delete
cleanup_if_empty() {
    local logfile="$1"
    # -s checks if file is non-empty; between check and rm, attacker
    # replaces the file with a symlink to a critical file.
    if [[ ! -s "$logfile" ]]; then
        rm -f "$logfile"  # vuln-code-snippet vuln-line race_test_then_delete
    fi
}
# vuln-code-snippet end race_test_then_delete

# vuln-code-snippet start race_pgrep_then_start
ensure_running() {
    local service="$1"
    # pgrep checks, then start — between the two, the process may have
    # already been started by another invocation, causing duplicate instances
    # or PID file corruption.
    if ! pgrep -x "$service" > /dev/null; then
        /usr/sbin/"$service" --daemon  # vuln-code-snippet vuln-line race_pgrep_then_start
    fi
}
# vuln-code-snippet end race_pgrep_then_start

# vuln-code-snippet start race_socket_then_connect
check_and_connect() {
    local port="$1"
    # Tests if port is open, then connects — between the check and the
    # connection, another process may bind the port (port hijacking).
    if nc -z localhost "$port" 2>/dev/null; then
        nc localhost "$port"  # vuln-code-snippet vuln-line race_socket_then_connect
    fi
}
# vuln-code-snippet end race_socket_then_connect

# vuln-code-snippet start race_glob_then_source
load_plugins() {
    local plugin_dir="$1"
    # Glob expansion then source — between ls and source, attacker
    # writes a malicious .sh file into the plugin directory.
    for plugin in "${plugin_dir}"/*.sh; do
        source "$plugin"  # vuln-code-snippet vuln-line race_glob_then_source
    done
}
# vuln-code-snippet end race_glob_then_source

# vuln-code-snippet start race_flock_fd_safe
exclusive_operation() {
    local lock="/var/run/myapp.lock"
    # flock on a file descriptor — atomic kernel-level lock, no TOCTOU.
    exec 9>"$lock"
    if flock -n 9; then
        do_critical_work  # vuln-code-snippet safe-line race_flock_fd_safe
        flock -u 9
    else
        echo "Another instance is running" >&2
        return 1
    fi
}
# vuln-code-snippet end race_flock_fd_safe

# vuln-code-snippet start race_atomic_symlink
update_config_atomic() {
    local new_config="$1"
    local link_path="/etc/app/current_config"
    local tmp_link
    tmp_link=$(mktemp -u /etc/app/.config.XXXXXX)
    # ln -sf + mv = atomic symlink update. mv on the same filesystem
    # is a single rename(2) syscall — no TOCTOU window.
    ln -sf "$new_config" "$tmp_link"
    mv -fT "$tmp_link" "$link_path"  # vuln-code-snippet safe-line race_atomic_symlink
}
# vuln-code-snippet end race_atomic_symlink

# vuln-code-snippet start race_mv_atomic_rename
write_then_rename() {
    local data="$1"
    local dest="$2"
    local tmp
    tmp=$(mktemp "${dest}.XXXXXX")
    echo "$data" > "$tmp"
    # mv (rename) is atomic on the same filesystem — readers either
    # see the old file or the new file, never a partial write.
    mv -f "$tmp" "$dest"  # vuln-code-snippet safe-line race_mv_atomic_rename
}
# vuln-code-snippet end race_mv_atomic_rename

# vuln-code-snippet start race_inotifywait_event
wait_for_file_event() {
    local watch_dir="$1"
    # inotifywait blocks on kernel inotify events — no polling loop,
    # no TOCTOU gap. Reacts to the actual filesystem event.
    inotifywait -e create -e modify "$watch_dir" && \
        process_new_files "$watch_dir"  # vuln-code-snippet safe-line race_inotifywait_event
}
# vuln-code-snippet end race_inotifywait_event

# vuln-code-snippet start race_open_excl_helper
create_exclusive_file() {
    local path="$1"
    local data="$2"
    # Python's O_CREAT|O_EXCL is the kernel-level atomic exclusive
    # create — fails if the file already exists (symlink or not).
    python3 -c '
import os, sys
fd = os.open(sys.argv[1], os.O_CREAT | os.O_EXCL | os.O_WRONLY, 0o600)
os.write(fd, sys.argv[2].encode())
os.close(fd)
' "$path" "$data"  # vuln-code-snippet safe-line race_open_excl_helper
}
# vuln-code-snippet end race_open_excl_helper

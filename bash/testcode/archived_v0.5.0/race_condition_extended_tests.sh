#!/bin/bash
# Race Condition Extended Test Cases (CWE-362)
# Additional patterns beyond v0.4.0: PID file TOCTOU, directory lock races,
# hosts file race, chmod window, crontab temp race, loop-then-touch TOCTOU.

# vuln-code-snippet start race_pid_file_write_gap
write_pidfile_checked() {
    # The check ([ ! -f "$pidfile" ]) and the write (echo $$) are two
    # separate operations with a gap between them. Another process can create
    # the pidfile between the check and the write. Both processes believe they
    # are the sole owner of the pidfile — a classic TOCTOU on PID file creation.
    local pidfile="/var/run/app.pid"
    if [ ! -f "$pidfile" ]; then
        echo $$ > "$pidfile"  # vuln-code-snippet vuln-line race_pid_file_write_gap
    fi
}
# vuln-code-snippet end race_pid_file_write_gap

# vuln-code-snippet start race_dir_lock_mkdir
acquire_dir_lock() {
    # ls (or [ -d ]) and mkdir are separate non-atomic operations. Multiple
    # concurrent processes can each see the directory absent and proceed to
    # mkdir — mkdir is not atomic in a multi-process context (only mkdir's
    # return code is the atomic signal). Use mkdir return code directly
    # without a preceding check.
    local lockdir="/tmp/app.lock"
    if ! ls "$lockdir" 2>/dev/null; then
        mkdir "$lockdir"  # vuln-code-snippet vuln-line race_dir_lock_mkdir
    fi
}
# vuln-code-snippet end race_dir_lock_mkdir

# vuln-code-snippet start race_hosts_append
add_hosts_entry() {
    # grep reads /etc/hosts at one point in time; the append occurs later.
    # Concurrent calls can each find the entry absent and each append it,
    # creating duplicate /etc/hosts entries. For critical system files,
    # use flock to serialize access.
    local host_entry="$1"
    if ! grep -qF "$host_entry" /etc/hosts; then
        echo "$host_entry" >> /etc/hosts  # vuln-code-snippet vuln-line race_hosts_append
    fi
}
# vuln-code-snippet end race_hosts_append

# vuln-code-snippet start race_permission_window
write_shared_data() {
    # There is a window between chmod 777 (world-writable) and chmod 755
    # where any local user can create, delete, or modify files in shared_dir.
    # An attacker replaces output.txt with a symlink during this window,
    # causing the subsequent write to target an arbitrary path.
    local shared_dir="/var/app/shared"
    local data="$1"
    chmod 777 "$shared_dir"  # vuln-code-snippet vuln-line race_permission_window
    echo "$data" > "${shared_dir}/output.txt"
    chmod 755 "$shared_dir"
}
# vuln-code-snippet end race_permission_window

# vuln-code-snippet start race_crontab_tmp
add_cron_job() {
    # /tmp/cron.tmp is a predictable path. A concurrent process or an attacker
    # can: (1) create a symlink at /tmp/cron.tmp before crontab -l writes,
    # redirecting the crontab dump elsewhere, or (2) modify the file between
    # crontab -l and crontab installation, injecting malicious cron entries.
    local cron_expr="$1"
    local cron_cmd="$2"
    crontab -l > /tmp/cron.tmp  # vuln-code-snippet vuln-line race_crontab_tmp
    echo "${cron_expr} ${cron_cmd}" >> /tmp/cron.tmp
    crontab /tmp/cron.tmp
}
# vuln-code-snippet end race_crontab_tmp

# vuln-code-snippet start race_loop_then_touch
wait_and_lock() {
    # The while loop exits when the lock file is absent, then touch creates it.
    # Multiple processes can exit the loop at approximately the same time (all
    # see the file absent) and proceed to touch — touch is not O_EXCL atomic.
    # All believe they hold the lock simultaneously.
    local LOCK="/var/run/app.lock"
    while [ -f "$LOCK" ]; do
        sleep 0.1
    done
    touch "$LOCK"  # vuln-code-snippet vuln-line race_loop_then_touch
}
# vuln-code-snippet end race_loop_then_touch

# vuln-code-snippet start race_config_check_cp
backup_config() {
    # The existence check and cp are separate. Another process may delete
    # config.bak between the check and the copy, or replace it with a symlink.
    # The cp then follows the symlink, writing config.conf contents to an
    # attacker-chosen path.
    local config="/etc/app/config.conf"
    if [ -f "${config}.bak" ]; then
        cp "$config" "${config}.bak"  # vuln-code-snippet vuln-line race_config_check_cp
    fi
}
# vuln-code-snippet end race_config_check_cp

# vuln-code-snippet start race_socket_check_create
create_unix_socket() {
    # The rm and socat are separate operations with a TOCTOU gap. Between rm
    # and socat's bind, an attacker creates a file or symlink at /tmp/app.sock.
    # socat's bind may fail or follow the symlink, depending on the socat
    # version and OS behavior.
    local socket="/tmp/app.sock"
    if [ -e "$socket" ]; then
        rm "$socket"
    fi
    socat "UNIX-LISTEN:${socket},fork" EXEC:"handle_req"  # vuln-code-snippet vuln-line race_socket_check_create
}
# vuln-code-snippet end race_socket_check_create

# vuln-code-snippet start race_find_cleanup_create
cleanup_and_create_lock() {
    # find -delete and touch are separate non-atomic operations. Another process
    # or attacker can create /tmp/app.lock (or a symlink at that path) between
    # the cleanup and the touch, causing the lock to be created at an
    # attacker-controlled target.
    find /tmp -name "*.lock" -mmin +5 -delete
    touch /tmp/app.lock  # vuln-code-snippet vuln-line race_find_cleanup_create
}
# vuln-code-snippet end race_find_cleanup_create

# vuln-code-snippet start race_cp_non_atomic
deploy_config() {
    # cp is not atomic — it truncates the destination, then writes. Between
    # truncation and completion, any process reading /etc/app/config.conf sees
    # an empty or partially-written file. For live configuration files, use
    # cp to a temp file then mv (atomic rename) instead.
    local new_config="$1"
    cp "$new_config" /etc/app/config.conf  # vuln-code-snippet vuln-line race_cp_non_atomic
}
# vuln-code-snippet end race_cp_non_atomic

# vuln-code-snippet start race_tar_live_dir
deploy_release() {
    # tar extraction is a multi-file non-atomic operation. During extraction,
    # the application may read partially-updated configurations (some files
    # from the old release, some from the new). For live directories, extract
    # to a staging directory and use atomic rename to swap.
    local archive="$1"
    tar xf "$archive" -C /etc/app/  # vuln-code-snippet vuln-line race_tar_live_dir
}
# vuln-code-snippet end race_tar_live_dir

# vuln-code-snippet start race_pgrep_start
ensure_daemon_running() {
    # pgrep and start_daemon are separate. Multiple invocations can each find
    # daemon absent and each call start_daemon — starting multiple daemon
    # instances. Daemon startup must use a lock (flock, pidfile) rather than
    # process existence checks.
    if [ -z "$(pgrep -x daemon_process)" ]; then
        start_daemon  # vuln-code-snippet vuln-line race_pgrep_start
    fi
}
# vuln-code-snippet end race_pgrep_start

# vuln-code-snippet start race_test_delete
cleanup_old_lock() {
    # The test-then-delete allows a symlink race: another process replaces
    # /var/run/app.tmp with a symlink to a sensitive file between the [ -f ]
    # check and rm. rm follows the symlink and deletes the target file.
    # Use rm -f directly (check return code) rather than testing first.
    local tmpfile="/var/run/app.tmp"
    if [ -f "$tmpfile" ]; then
        rm "$tmpfile"  # vuln-code-snippet vuln-line race_test_delete
    fi
    create_lockfile
}
# vuln-code-snippet end race_test_delete

# vuln-code-snippet start race_mkdir_chmod_window
create_private_directory() {
    # mkdir creates the directory with umask-based permissions (typically 755),
    # then chmod 700 restricts access. Between mkdir and chmod, the directory
    # is world-accessible. A concurrent process can create files inside the
    # directory during this window before permissions are restricted.
    local dir_name="$1"
    local target_dir="/var/app/${dir_name}"
    mkdir -p "$target_dir"  # vuln-code-snippet vuln-line race_mkdir_chmod_window
    chmod 700 "$target_dir"
}
# vuln-code-snippet end race_mkdir_chmod_window

# vuln-code-snippet start race_inotify_gap
watch_and_reload() {
    # There is a TOCTOU between inotifywait receiving the close_write event
    # and reload_service reading the configuration. Another process can replace
    # config_file with a malicious version between the event and the reload.
    # The file's content at event time is not the same as at reload time.
    local config_file="$1"
    inotifywait -e close_write "$config_file"
    reload_service  # vuln-code-snippet vuln-line race_inotify_gap
}
# vuln-code-snippet end race_inotify_gap

# --- Safe variants ---

# vuln-code-snippet start race_noclobber_create
try_acquire_lock() {
    # noclobber causes bash to use O_CREAT|O_EXCL when redirecting with >.
    # This is atomic — if the file already exists, the redirect fails.
    # The subshell scopes the noclobber setting to avoid affecting the parent shell.
    local lockfile="/var/run/app.lock"
    ( set -o noclobber; echo $$ > "$lockfile" ) 2>/dev/null || { echo "Already running" >&2; return 1; }  # vuln-code-snippet safe-line race_noclobber_create
}
# vuln-code-snippet end race_noclobber_create

# vuln-code-snippet start race_flock_command
run_with_lock() {
    # flock acquires an exclusive advisory lock using the flock(2) syscall.
    # -n returns immediately (non-blocking) if the lock is held by another
    # process. The entire do_critical_work command runs within the lock.
    flock -n /var/lock/app.lock -c "do_critical_work"  # vuln-code-snippet safe-line race_flock_command
}
# vuln-code-snippet end race_flock_command

# vuln-code-snippet start race_ln_atomic
create_atomic_lock() {
    # symlink creation is atomic per POSIX — it uses the link(2) or
    # symlinkat(2) syscall which either succeeds or fails atomically with
    # EEXIST. If the symlink already exists, ln fails, and the caller knows
    # the lock is held.
    local lockfile="/tmp/app.lock"
    ln -s $$ "$lockfile" 2>/dev/null || { echo "Lock held by another process" >&2; return 1; }  # vuln-code-snippet safe-line race_ln_atomic
    trap 'rm -f "$lockfile"' EXIT
}
# vuln-code-snippet end race_ln_atomic

# vuln-code-snippet start race_install_atomic_create
create_restricted_lockfile() {
    # install uses open(O_CREAT|O_EXCL) internally when creating new files.
    # The -m 600 sets permissions atomically at creation time — no window
    # between create and chmod.
    install -m 600 /dev/null /var/run/app.lock  # vuln-code-snippet safe-line race_install_atomic_create
}
# vuln-code-snippet end race_install_atomic_create

# vuln-code-snippet start race_mktemp_with_trap
setup_temp_workspace() {
    # mktemp uses open(O_CREAT|O_EXCL) — atomic creation with no TOCTOU gap.
    # The trap ensures cleanup on all exit paths.
    local tmpfile
    tmpfile=$(mktemp)  # vuln-code-snippet safe-line race_mktemp_with_trap
    trap 'rm -f "$tmpfile"' EXIT
}
# vuln-code-snippet end race_mktemp_with_trap

# vuln-code-snippet start race_flock_fd_wrapper
run_serialized_section() {
    # FD 9 is opened on the lock file (created if necessary). flock 9 blocks
    # until the exclusive lock is acquired on FD 9. The flock(2) syscall is
    # atomic — no TOCTOU gap exists between checking and acquiring the lock.
    (
        flock 9  # vuln-code-snippet safe-line race_flock_fd_wrapper
        perform_critical_operation
    ) 9>/var/lock/myapp.lock
}
# vuln-code-snippet end race_flock_fd_wrapper

# vuln-code-snippet start race_atomic_rename
update_config_atomically() {
    # mv (rename(2) syscall) is atomic on the same filesystem. Processes
    # reading /etc/app/config.conf see either the old or the new version,
    # never a partial write. This is the standard pattern for atomic config
    # updates.
    local new_config_content="$1"
    local tmp
    tmp=$(mktemp /etc/app/config.XXXXXX)
    echo "$new_config_content" > "$tmp"
    mv "$tmp" /etc/app/config.conf  # vuln-code-snippet safe-line race_atomic_rename
}
# vuln-code-snippet end race_atomic_rename

# vuln-code-snippet start race_install_perms
deploy_executable() {
    # install atomically copies, sets permissions, and sets ownership in a
    # single operation. Unlike cp followed by chmod followed by chown, there
    # is no window where the file has wrong permissions or ownership.
    local src="$1"
    install -m 755 -o root "$src" /usr/local/bin/app  # vuln-code-snippet safe-line race_install_perms
}
# vuln-code-snippet end race_install_perms

# vuln-code-snippet start race_mkdir_return_code
acquire_mkdir_lock() {
    # mkdir's return code IS the atomic signal — if mkdir succeeds, this
    # process created the directory; if it fails (EEXIST), another process
    # holds the lock. No preceding check is used; the mkdir operation itself
    # is the test-and-set.
    local lockdir="/tmp/app_run.lock"
    mkdir "$lockdir" 2>/dev/null && trap 'rmdir "$lockdir"' EXIT || { echo "Lock held" >&2; return 1; }  # vuln-code-snippet safe-line race_mkdir_return_code
}
# vuln-code-snippet end race_mkdir_return_code

# vuln-code-snippet start race_flock_timeout
run_with_timed_lock() {
    # flock with -w 30 waits up to 30 seconds to acquire the exclusive lock.
    # -x requests exclusive (write) lock on FD 200. The flock(2) syscall is
    # atomic. The timeout prevents indefinite blocking.
    (
        flock -x -w 30 200 || { echo "Could not acquire lock within 30s" >&2; exit 1; }  # vuln-code-snippet safe-line race_flock_timeout
        perform_deployment
    ) 200>/var/lock/deploy.lock
}
# vuln-code-snippet end race_flock_timeout

# vuln-code-snippet start race_mktemp_private
setup_private_workspace() {
    # mktemp -d creates the directory atomically with default 700 permissions.
    # chmod 700 is redundant here (mktemp -d already creates with 700) but
    # does not introduce a race — the directory is already private before
    # chmod runs.
    local TMPDIR_PRIV
    TMPDIR_PRIV=$(mktemp -d)  # vuln-code-snippet safe-line race_mktemp_private
    chmod 700 "$TMPDIR_PRIV"
}
# vuln-code-snippet end race_mktemp_private

# vuln-code-snippet start race_write_atomic_func
write_atomic() {
    # This is the canonical atomic write pattern: write to a unique temp file,
    # then rename to the target. The rename(2) syscall is atomic on the same
    # filesystem. Readers see either the old or new content, never a partial
    # write.
    local target="$1"
    local content="$2"
    local tmp
    tmp=$(mktemp "${target}.XXXXXX")
    printf '%s' "$content" > "$tmp"
    mv "$tmp" "$target"  # vuln-code-snippet safe-line race_write_atomic_func
}
# vuln-code-snippet end race_write_atomic_func

# vuln-code-snippet start race_noclobber_loop
acquire_lock_with_retry() {
    # The noclobber in a subshell uses O_CREAT|O_EXCL — atomic. The loop
    # retries safely (no TOCTOU in the acquire step itself). The sleep prevents
    # busy-waiting. This is a correct retry lock acquisition pattern.
    local lockfile="/var/run/app.lock"
    while true; do
        ( set -o noclobber; echo $$ > "$lockfile" ) 2>/dev/null && break  # vuln-code-snippet safe-line race_noclobber_loop
        sleep 1
    done
    trap 'rm -f "$lockfile"' EXIT
}
# vuln-code-snippet end race_noclobber_loop

# vuln-code-snippet start race_unlink_after_open
use_anonymous_temp() {
    # The file is unlinked immediately after FD 3 is opened. The FD keeps the
    # file alive in memory but removes it from the directory. No other process
    # can find or open the file via its path — it is effectively anonymous.
    # No race condition is possible on a path that no longer exists.
    local data="$1"
    local TMPFILE
    TMPFILE=$(mktemp)
    exec 3>"$TMPFILE"
    rm "$TMPFILE"  # vuln-code-snippet safe-line race_unlink_after_open
    echo "$data" >&3
    exec 3>&-
}
# vuln-code-snippet end race_unlink_after_open

# vuln-code-snippet start race_systemd_serial
run_exclusive_deployment() {
    # systemd's unit activation is serialized by the service manager. --wait
    # blocks until the service completes. systemd prevents concurrent activation
    # of units with the same name (if a fixed unit name is used), providing
    # OS-level serialization without application-level locking.
    local unit_name
    unit_name="deploy-$(date +%s).service"
    systemd-run --wait --unit="$unit_name" /var/app/bin/deploy.sh  # vuln-code-snippet safe-line race_systemd_serial
}
# vuln-code-snippet end race_systemd_serial

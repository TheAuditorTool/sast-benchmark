#!/bin/bash
# Insecure Temporary File Extended Test Cases (CWE-377/367)
# Additional patterns beyond v0.4.0: script-name paths, hostname locks,
# whoami-based paths, /dev/shm fixed names, PID key races, mktemp reinventions.

# vuln-code-snippet start insecure_temp_script_name
create_work_temp() {
    # ${0##*/} expands to the script's basename (e.g., deploy.sh -> deploy.sh.tmp).
    # This name is entirely predictable from the script filename. An attacker can
    # pre-create a symlink at /tmp/deploy.sh.tmp pointing to a sensitive file before
    # the script runs, causing subsequent writes to overwrite that file.
    local TMPFILE="/tmp/${0##*/}.tmp"  # vuln-code-snippet vuln-line insecure_temp_script_name
    touch "$TMPFILE"
    echo "$TMPFILE"
}
# vuln-code-snippet end insecure_temp_script_name

# vuln-code-snippet start insecure_temp_hostname_lock
acquire_run_lock() {
    # The hostname is a public, non-secret value (visible in DNS, network scans, prompt).
    # Any user or script on the system or network that knows the hostname can pre-create
    # a symlink or file at this path to interfere with the locking mechanism.
    # shellcheck disable=SC2155
    local LOCK="/tmp/$(hostname).lock"  # vuln-code-snippet vuln-line insecure_temp_hostname_lock
    touch "$LOCK"
    echo "Lock acquired: $LOCK"
}
# vuln-code-snippet end insecure_temp_hostname_lock

# vuln-code-snippet start insecure_temp_whoami_path
create_user_cache() {
    # The username is predictable and publicly known. Other users on the system can
    # observe the naming pattern and pre-create a symlink at /tmp/output_username to
    # redirect writes. /tmp paths based solely on username provide no collision resistance.
    # shellcheck disable=SC2155
    local tmpfile="/tmp/output_$(whoami)"  # vuln-code-snippet vuln-line insecure_temp_whoami_path
    echo "cache data" > "$tmpfile"
}
# vuln-code-snippet end insecure_temp_whoami_path

# vuln-code-snippet start insecure_temp_service_name
write_service_cache() {
    local data="$1"
    # A fully fixed cache path provides zero collision resistance. Any local user can
    # create /tmp/myapp-cache before the service runs. This is a symlink attack surface
    # — the service will follow the symlink and write to wherever it points.
    local CACHE_FILE="/tmp/myapp-cache"  # vuln-code-snippet vuln-line insecure_temp_service_name
    echo "$data" > "$CACHE_FILE"
}
# vuln-code-snippet end insecure_temp_service_name

# vuln-code-snippet start insecure_temp_devshm_fixed
store_session_data() {
    local session_data="$1"
    # /dev/shm is a tmpfs mount (world-readable by default on many systems). The fixed
    # filename app_session has zero entropy. Other processes and users with access to
    # /dev/shm can read or overwrite the session data.
    echo "$session_data" > /dev/shm/app_session  # vuln-code-snippet vuln-line insecure_temp_devshm_fixed
}
# vuln-code-snippet end insecure_temp_devshm_fixed

# vuln-code-snippet start insecure_temp_fixed_config
write_temp_config() {
    local db_pass="$1"
    # Writing secrets (db_pass) to a fixed /tmp path creates a world-readable file with
    # a predictable name. Both the secret disclosure (world-readable) and the symlink
    # attack surface (predictable name) are vulnerabilities. Use mktemp with immediate
    # chmod 600.
    local CONFIG_TMP="/tmp/app_config.json"  # vuln-code-snippet vuln-line insecure_temp_fixed_config
    cat > "$CONFIG_TMP" << EOF
{"db_password": "${db_pass}"}
EOF
}
# vuln-code-snippet end insecure_temp_fixed_config

# vuln-code-snippet start insecure_temp_port_path
create_socket_file() {
    local port="$1"
    # Port numbers are enumerable (1-65535) and often public knowledge. The named pipe
    # path /tmp/pipe_PORT is predictable. An attacker who knows the port can pre-create
    # a file at this path, preventing mkfifo from creating the FIFO (it fails on
    # existing files) — a DoS against the application.
    local PIPE_FILE="/tmp/pipe_${port}"  # vuln-code-snippet vuln-line insecure_temp_port_path
    mkfifo "$PIPE_FILE"
}
# vuln-code-snippet end insecure_temp_port_path

# vuln-code-snippet start insecure_temp_fixed_build_dir
prepare_build_environment() {
    # A fixed build directory /tmp/build is predictable. If an attacker pre-creates
    # /tmp/build as a symlink to a sensitive directory (e.g., /etc/cron.d/), subsequent
    # build operations (copying files, running make install) affect the attacker-chosen
    # target.
    local tmpdir="/tmp/build"  # vuln-code-snippet vuln-line insecure_temp_fixed_build_dir
    mkdir -p "$tmpdir"
    echo "Build directory: $tmpdir"
}
# vuln-code-snippet end insecure_temp_fixed_build_dir

# vuln-code-snippet start insecure_temp_argv0
setup_logging() {
    # $0 is the full path to the script (e.g., /opt/app/scripts/deploy.sh). Using the
    # full path as a temp filename creates paths that may not resolve correctly, and $0
    # with basename is entirely predictable. The log file is also often world-readable
    # if created in /tmp.
    local LOG_TMP="/tmp/${0}.log"  # vuln-code-snippet vuln-line insecure_temp_argv0
    exec 2>>"$LOG_TMP"
}
# vuln-code-snippet end insecure_temp_argv0

# vuln-code-snippet start insecure_temp_pid_key_race
generate_ephemeral_key() {
    # CWE-367 TOCTOU: the PID-based temp path is predictable before the script runs
    # (PIDs are sequential and observable). Between name construction and openssl's
    # creation of the file, an attacker creates a symlink at /tmp/key_PID pointing to
    # another file. openssl writes the generated private key to the symlink target,
    # overwriting it.
    local KEY_TEMP="/tmp/key_$$"  # vuln-code-snippet vuln-line insecure_temp_pid_key_race
    openssl genrsa -out "$KEY_TEMP" 2048 2>/dev/null
    echo "$KEY_TEMP"
}
# vuln-code-snippet end insecure_temp_pid_key_race

# vuln-code-snippet start insecure_temp_reinvent_mktemp
make_secure_temp() {
    local timestamp
    timestamp=$(date +%s)
    local f="/tmp/app.${timestamp}.$$"
    # CWE-367 TOCTOU: the path is constructed from predictable inputs (timestamp + PID),
    # then touch creates it. Between path construction and touch execution, an attacker
    # can create a symlink at the predicted path. Use mktemp which uses
    # O_CREAT|O_EXCL atomically.
    touch "$f"  # vuln-code-snippet vuln-line insecure_temp_reinvent_mktemp
    echo "$f"
}
# vuln-code-snippet end insecure_temp_reinvent_mktemp

# vuln-code-snippet start insecure_temp_fallback_pid
get_temp_path() {
    # When mktemp fails, the fallback is a PID-based path /tmp/app_tmp_PID. PIDs are
    # predictable and the fallback provides no security. If mktemp fails (e.g., no
    # space in TMPDIR), the code silently uses a vulnerable path. The fallback should
    # exit on mktemp failure instead.
    local tmpfile
    tmpfile=$(mktemp 2>/dev/null || echo "/tmp/app_tmp_$$")  # vuln-code-snippet vuln-line insecure_temp_fallback_pid
    echo "$tmpfile"
}
# vuln-code-snippet end insecure_temp_fallback_pid

# vuln-code-snippet start insecure_temp_rm_mkdir_gap
reset_work_directory() {
    local WORK_DIR="/tmp/work"
    rm -rf "$WORK_DIR"
    # CWE-367 TOCTOU: between rm -rf and mkdir, an attacker creates a symlink at
    # /tmp/work pointing to /etc/app/. The subsequent mkdir fails (symlink exists), and
    # subsequent operations in the "work directory" follow the symlink to /etc/app/.
    mkdir "$WORK_DIR"  # vuln-code-snippet vuln-line insecure_temp_rm_mkdir_gap
}
# vuln-code-snippet end insecure_temp_rm_mkdir_gap

# vuln-code-snippet start insecure_temp_var_tmp
create_staging_area() {
    local app_name="$1"
    # /var/tmp is persistent across reboots (unlike /tmp which may be cleared). Files
    # there accumulate. The path is predictable from app_name (often known or guessable).
    # Persistent predictable paths in /var/tmp are an elevated risk compared to /tmp
    # because they survive longer.
    local STAGING="/var/tmp/${app_name}_staging"  # vuln-code-snippet vuln-line insecure_temp_var_tmp
    mkdir -p "$STAGING"
    echo "$STAGING"
}
# vuln-code-snippet end insecure_temp_var_tmp

# vuln-code-snippet start insecure_temp_race_on_generate
make_random_temp() {
    local n
    n=$(head -c 4 /dev/urandom | xxd -p | tr -d '\n')
    local f="/tmp/${n}"
    # CWE-367 TOCTOU: although n comes from /dev/urandom (genuinely random), there is
    # still a race between constructing the path and touch creating it. The
    # O_CREAT|O_EXCL atomic guarantee of mktemp is absent — touch uses open() without
    # O_EXCL, allowing symlink injection between n generation and the touch call.
    touch "$f"  # vuln-code-snippet vuln-line insecure_temp_race_on_generate
    echo "$f"
}
# vuln-code-snippet end insecure_temp_race_on_generate

# vuln-code-snippet start insecure_temp_mktemp_prefix
create_prefixed_temp() {
    # mktemp -t uses at least 10 random characters (XXXXXXXXXX) from the OS entropy
    # source. The O_CREAT|O_EXCL flag makes creation atomic — no TOCTOU race is possible.
    local TMPFILE
    TMPFILE=$(mktemp -t "appname.XXXXXXXXXX")  # vuln-code-snippet safe-line insecure_temp_mktemp_prefix
    echo "$TMPFILE"
}
# vuln-code-snippet end insecure_temp_mktemp_prefix

# vuln-code-snippet start insecure_temp_mktemp_tmpdir
create_work_dir_respecting_tmpdir() {
    # Respects the $TMPDIR environment variable (allows sysadmins to point tmp to a
    # more secure location). mktemp -d creates the directory atomically with 700
    # permissions.
    local TMPDIR_WORK
    TMPDIR_WORK=$(mktemp -d "${TMPDIR:-/tmp}/app.XXXXXXXXXX")  # vuln-code-snippet safe-line insecure_temp_mktemp_tmpdir
    echo "$TMPDIR_WORK"
}
# vuln-code-snippet end insecure_temp_mktemp_tmpdir

# vuln-code-snippet start insecure_temp_mktemp_full_trap
setup_work_environment() {
    # mktemp -d creates a unique temporary directory. The trap ensures cleanup on all
    # exit paths including SIGINT and SIGTERM, preventing temp directory accumulation.
    local WORKDIR
    WORKDIR=$(mktemp -d)  # vuln-code-snippet safe-line insecure_temp_mktemp_full_trap
    trap 'rm -rf "$WORKDIR"' INT TERM EXIT
    echo "$WORKDIR"
}
# vuln-code-snippet end insecure_temp_mktemp_full_trap

# vuln-code-snippet start insecure_temp_mktemp_chmod
create_restricted_temp() {
    local secret="$1"
    # mktemp creates the file atomically (O_CREAT|O_EXCL). chmod 600 immediately
    # restricts access before the secret is written, preventing other users from reading
    # during the window between creation and data write.
    local TMPFILE
    TMPFILE=$(mktemp)  # vuln-code-snippet safe-line insecure_temp_mktemp_chmod
    chmod 600 "$TMPFILE"
    echo "$secret" > "$TMPFILE"
}
# vuln-code-snippet end insecure_temp_mktemp_chmod

# vuln-code-snippet start insecure_temp_python_mkstemp
create_secure_python_temp() {
    # Python's tempfile.mkstemp() uses os.open() with O_CREAT|O_EXCL — atomic creation.
    # The file descriptor is immediately chmod'd to 600 before closing.
    # No TOCTOU race possible.
    local secret_path
    secret_path=$(python3 -c "
import tempfile, os
fd, path = tempfile.mkstemp(suffix='.key')
os.chmod(path, 0o600)
os.close(fd)
print(path)
")  # vuln-code-snippet safe-line insecure_temp_python_mkstemp
    echo "$secret_path"
}
# vuln-code-snippet end insecure_temp_python_mkstemp

# vuln-code-snippet start insecure_temp_mktemp_suffix_key
generate_key_file() {
    # mktemp with --suffix adds a recognizable extension while maintaining randomness in
    # the base name. chmod 400 immediately makes the file read-only by owner only before
    # any key material is written.
    local SECRET_FILE
    SECRET_FILE=$(mktemp --suffix=.key)  # vuln-code-snippet safe-line insecure_temp_mktemp_suffix_key
    chmod 400 "$SECRET_FILE"
    echo "$SECRET_FILE"
}
# vuln-code-snippet end insecure_temp_mktemp_suffix_key

# vuln-code-snippet start insecure_temp_unlink_after_open
use_ephemeral_file() {
    local data="$1"
    local TMPFILE
    TMPFILE=$(mktemp)
    exec 3>"$TMPFILE"
    # The file is unlinked immediately after opening — the file descriptor keeps it
    # alive in memory, but no filesystem path remains. Other processes cannot find the
    # file via /tmp, making it effectively anonymous. This is the most secure pattern
    # for short-lived sensitive temp data.
    rm "$TMPFILE"  # vuln-code-snippet safe-line insecure_temp_unlink_after_open
    echo "$data" >&3
    exec 3>&-
}
# vuln-code-snippet end insecure_temp_unlink_after_open

# vuln-code-snippet start insecure_temp_shred_on_exit
store_temp_key() {
    local key_material="$1"
    # mktemp provides a unique path. shred -u on EXIT overwrites the file contents
    # before unlinking, preventing recovery from storage media. This is appropriate
    # when the temp file contains cryptographic key material.
    local tmpfile
    tmpfile=$(mktemp)  # vuln-code-snippet safe-line insecure_temp_shred_on_exit
    trap 'shred -u "$tmpfile"' EXIT
    echo "$key_material" > "$tmpfile"
}
# vuln-code-snippet end insecure_temp_shred_on_exit

# vuln-code-snippet start insecure_temp_private_tmpfs
mount_private_workspace() {
    local tmpdir
    tmpdir=$(mktemp -d)
    # A private tmpfs mount with mode=700 is accessible only to root (since mount
    # requires root). The size limit prevents resource exhaustion. The entire mount is
    # removed on exit, leaving no traces on disk.
    mount -t tmpfs -o "size=64m,mode=700" tmpfs "$tmpdir"  # vuln-code-snippet safe-line insecure_temp_private_tmpfs
    # shellcheck disable=SC2064
    trap "umount '$tmpdir' && rmdir '$tmpdir'" EXIT
    echo "$tmpdir"
}
# vuln-code-snippet end insecure_temp_private_tmpfs

# vuln-code-snippet start insecure_temp_mktemp_unique_name
create_named_pipe() {
    local FIFO
    FIFO=$(mktemp -u "/tmp/app.XXXXXXXXXX.sock")
    # mktemp -u generates a unique name without creating the file (unsafe for regular
    # files, but necessary for mkfifo which needs to create the special file type itself).
    # -m 600 on mkfifo creates the FIFO with restricted permissions atomically.
    mkfifo -m 600 "$FIFO"  # vuln-code-snippet safe-line insecure_temp_mktemp_unique_name
    echo "$FIFO"
}
# vuln-code-snippet end insecure_temp_mktemp_unique_name

# vuln-code-snippet start insecure_temp_ci_uuid_name
write_ci_build_log() {
    local log_content="$1"
    # Discrimination test: CI_JOB_ID is set by the CI/CD system (GitLab CI, GitHub
    # Actions) as a UUID (e.g., 550e8400-e29b-41d4-a716-446655440000). Unlike PID or
    # timestamp, a UUID provides 122 bits of randomness — equivalent to mktemp entropy.
    # Tools that flag all non-mktemp /tmp paths without checking entropy source FP here.
    local BUILD_LOG="/tmp/${CI_JOB_ID}.log"  # vuln-code-snippet safe-line insecure_temp_ci_uuid_name
    echo "$log_content" > "$BUILD_LOG"
}
# vuln-code-snippet end insecure_temp_ci_uuid_name

# vuln-code-snippet start insecure_temp_known_static_nonsecret
write_build_scratchpad() {
    local build_output="$1"
    # Discrimination test: build_output is non-sensitive compilation output (not
    # credentials or keys). The fixed path is intentional for a CI scratchpad that is
    # read back immediately and discarded. Context determines sensitivity — not every
    # /tmp write is a security issue.
    echo "$build_output" > /tmp/build_output_scratchpad  # vuln-code-snippet safe-line insecure_temp_known_static_nonsecret
}
# vuln-code-snippet end insecure_temp_known_static_nonsecret

# vuln-code-snippet start insecure_temp_systemd_tmp
get_managed_tmpdir() {
    # systemd-run with --tmpdir creates a managed temporary directory under
    # /run/user/$UID with correct permissions. The directory is automatically cleaned up
    # when the systemd scope exits, with kernel-enforced cleanup.
    local TMPDIR
    TMPDIR=$(systemd-run --quiet --user --wait --tmpdir --scope /bin/true)  # vuln-code-snippet safe-line insecure_temp_systemd_tmp
    echo "$TMPDIR"
}
# vuln-code-snippet end insecure_temp_systemd_tmp

# vuln-code-snippet start insecure_temp_mktemp_u_socket
create_unix_socket() {
    # mktemp -u generates a unique socket path name. socat's mode=600 creates the
    # socket file with restricted permissions, accessible only by the process owner.
    local SOCKET
    SOCKET=$(mktemp -u "/tmp/app.XXXXXXXXXX.sock")  # vuln-code-snippet safe-line insecure_temp_mktemp_u_socket
    socat "UNIX-LISTEN:${SOCKET},fork,mode=600" EXEC:"handle_request"
}
# vuln-code-snippet end insecure_temp_mktemp_u_socket

# vuln-code-snippet start insecure_temp_mktemp_in_subshell
perform_locked_operation() {
    local data="$1"
    # mktemp creates the lock file uniquely. flock serializes access across concurrent
    # invocations. The lockfile is cleaned up after the locked operation completes.
    # No predictable path exists for an attacker to target.
    local LOCKFILE
    LOCKFILE=$(mktemp)  # vuln-code-snippet safe-line insecure_temp_mktemp_in_subshell
    ( flock "$LOCKFILE"; echo "$data" > /var/app/data.conf )
    rm -f "$LOCKFILE"
}
# vuln-code-snippet end insecure_temp_mktemp_in_subshell

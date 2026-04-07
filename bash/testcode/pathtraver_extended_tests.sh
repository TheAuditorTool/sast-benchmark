#!/bin/bash
# Path Traversal Extended Test Cases (CWE-22)
# Additional patterns beyond v0.4.0: cp/head/diff with user paths, source with env
# component injection, readarray/jq on user paths, cpio/patch, FD-based reads.

# vuln-code-snippet start pathtraver_cp_user_src
backup_user_file() {
    local user_file="$1"
    # cp reads from user_file and writes to /var/backups/. With user_file set to
    # /etc/shadow, /etc/ssl/private/key.pem, or any sensitive path, cp reads the
    # file's contents and copies them to an attacker-accessible backup location.
    # No path validation is performed.
    cp "$user_file" /var/backups/  # vuln-code-snippet vuln-line pathtraver_cp_user_src
}
# vuln-code-snippet end pathtraver_cp_user_src

# vuln-code-snippet start pathtraver_head_user_log
preview_log_file() {
    local log_path="$1"
    # head reads from log_path and outputs to stdout. Supplying /etc/passwd,
    # /etc/shadow, or any readable system file outputs the first 20 lines of that
    # file. The intent is log preview, but the function reads arbitrary paths
    # without validation.
    head -n 20 "$log_path"  # vuln-code-snippet vuln-line pathtraver_head_user_log
}
# vuln-code-snippet end pathtraver_head_user_log

# vuln-code-snippet start pathtraver_diff_user_file
compare_config() {
    local user_config="$1"
    # diff reads user_config and outputs the differences against config.default.
    # If user_config is set to /etc/shadow, the shadow file contents are disclosed
    # in the diff output (lines prefixed with <). Any readable file can be
    # exfiltrated this way.
    diff "$user_config" /etc/app/config.default  # vuln-code-snippet vuln-line pathtraver_diff_user_file
}
# vuln-code-snippet end pathtraver_diff_user_file

# vuln-code-snippet start pathtraver_find_cat_user_dir
dump_config_files() {
    local user_dir="$1"
    # find traverses user_dir recursively. Supplying / or /etc causes find to
    # search the entire filesystem. -exec cat dumps the contents of every .conf
    # file found, including sensitive system configurations and credentials.
    find "$user_dir" -name "*.conf" -exec cat {} \;  # vuln-code-snippet vuln-line pathtraver_find_cat_user_dir
}
# vuln-code-snippet end pathtraver_find_cat_user_dir

# vuln-code-snippet start pathtraver_zip_user_src
archive_user_content() {
    local user_src="$1"
    # zip recursively archives user_src. Supplying /etc, /home, or /var/secrets
    # archives sensitive directories. The resulting zip file is world-readable in
    # /tmp, making the archived contents accessible to all local users.
    zip -r /tmp/user_archive.zip "$user_src"  # vuln-code-snippet vuln-line pathtraver_zip_user_src
}
# vuln-code-snippet end pathtraver_zip_user_src

# vuln-code-snippet start pathtraver_source_env_scope
load_environment_config() {
    local user_env="$1"
    local CONFIG_DIR="/etc/app/environments"
    # user_env is interpolated into the middle of the path. Supplying
    # ../../etc/cron.d/evil (without the .conf extension) or
    # ../../../proc/self/environ traverses out of /etc/app/environments and
    # sources an attacker-chosen file. The fixed prefix and suffix do not prevent
    # traversal through the middle component.
    source "${CONFIG_DIR}/${user_env}.conf"  # vuln-code-snippet vuln-line pathtraver_source_env_scope
}
# vuln-code-snippet end pathtraver_source_env_scope

# vuln-code-snippet start pathtraver_rsync_user_src
stage_content() {
    local user_path="$1"
    # rsync copies files from user_path to the staging directory. Supplying /etc/
    # or /var/secrets/ stages sensitive system files. The trailing slash means
    # rsync copies directory contents, potentially staging credentials and keys
    # alongside legitimate files.
    rsync -av "${user_path}/" /tmp/staging/  # vuln-code-snippet vuln-line pathtraver_rsync_user_src
}
# vuln-code-snippet end pathtraver_rsync_user_src

# vuln-code-snippet start pathtraver_python_exec_path
run_plugin() {
    local script_path="$1"
    # python3 executes script_path as a Python script. Supplying an
    # attacker-controlled path allows execution of arbitrary Python code. Combined
    # with path traversal, the attacker can point to a script in any readable
    # location — including symlinked paths or world-writable directories.
    python3 "$script_path"  # vuln-code-snippet vuln-line pathtraver_python_exec_path
}
# vuln-code-snippet end pathtraver_python_exec_path

# vuln-code-snippet start pathtraver_readarray_user
load_allowlist() {
    local list_file="$1"
    # readarray reads all lines from list_file into the ALLOWLIST array. Supplying
    # /etc/shadow loads shadow file entries into the array, where they may be
    # logged, output to stderr, or processed in ways that disclose their contents.
    # Any readable file is accessible.
    readarray -t ALLOWLIST < "$list_file"; export ALLOWLIST  # vuln-code-snippet vuln-line pathtraver_readarray_user
}
# vuln-code-snippet end pathtraver_readarray_user

# vuln-code-snippet start pathtraver_jq_user_json
parse_config_file() {
    local json_path="$1"
    # jq parses json_path as JSON and outputs the selected field. Supplying a
    # non-JSON system file causes jq to output a parse error that includes file
    # contents. Even for valid JSON files outside the intended directory, jq leaks
    # their contents via the matched query result.
    jq '.database.host' "$json_path"  # vuln-code-snippet vuln-line pathtraver_jq_user_json
}
# vuln-code-snippet end pathtraver_jq_user_json

# vuln-code-snippet start pathtraver_base_path_escape
read_report_file() {
    local user_path="$1"
    local BASE="/var/app/reports"
    # Although BASE is hardcoded, user_path is appended without validation.
    # Supplying ../../etc/shadow produces /var/app/reports/../../etc/shadow which
    # resolves to /etc/shadow. String concatenation with a fixed prefix does not
    # prevent path traversal — canonicalization via realpath is required.
    cat "${BASE}/${user_path}"  # vuln-code-snippet vuln-line pathtraver_base_path_escape
}
# vuln-code-snippet end pathtraver_base_path_escape

# vuln-code-snippet start pathtraver_fd_read_user
stream_file_contents() {
    local user_file="$1"
    # Opening the file descriptor on user_file reads from an arbitrary path. The
    # file is streamed line-by-line and echoed to stdout. Supplying /etc/shadow or
    # any sensitive file exfiltrates its contents. The FD-based approach bypasses
    # simple pattern matchers that look for `cat` or `head` but has identical effect.
    exec 3< "$user_file"  # vuln-code-snippet vuln-line pathtraver_fd_read_user
    while IFS= read -u 3 -r line; do
        echo "$line"
    done
    exec 3<&-
}
# vuln-code-snippet end pathtraver_fd_read_user

# vuln-code-snippet start pathtraver_patch_apply_user
apply_user_patch() {
    local patch_file="$1"
    # patch reads hunks from patch_file and applies them to files specified within
    # the patch. An attacker-crafted patch file can target any writable path (e.g.,
    # +++ /etc/cron.d/evil). The -p1 strip removes one leading path component, but
    # the patch can still traverse to arbitrary locations relative to the working
    # directory.
    patch -p1 < "$patch_file"  # vuln-code-snippet vuln-line pathtraver_patch_apply_user
}
# vuln-code-snippet end pathtraver_patch_apply_user

# vuln-code-snippet start pathtraver_cpio_user_archive
extract_cpio_archive() {
    local archive_path="$1"
    # cpio extracts to paths specified within the archive. A malicious archive can
    # contain entries with path traversal sequences (e.g., ../../etc/cron.d/evil)
    # that write outside the current directory. Unlike tar --strip-components, cpio
    # does not have a --no-absolute-filenames equivalent enforced by default in all
    # implementations.
    cpio -idv < "$archive_path"  # vuln-code-snippet vuln-line pathtraver_cpio_user_archive
}
# vuln-code-snippet end pathtraver_cpio_user_archive

# vuln-code-snippet start pathtraver_env_override_source
initialize_from_config() {
    # USER_ENV defaults to "production" and appears safe in normal operation, but
    # USER_ENV is an environment variable that callers can override. Setting
    # USER_ENV=../../etc/cron.d causes source to execute an attacker-controlled
    # path. The :-production default only protects when USER_ENV is unset, not
    # when it is set to a malicious value.
    local CONFIG_PATH="${APP_DIR}/${USER_ENV:-production}/config.sh"
    source "$CONFIG_PATH"  # vuln-code-snippet vuln-line pathtraver_env_override_source
}
# vuln-code-snippet end pathtraver_env_override_source

# --- Safe variants ---

# vuln-code-snippet start pathtraver_log_id_validated
read_log_by_id() {
    local log_id="$1"
    # The regex ^[0-9a-f]{8}$ permits only 8 lowercase hexadecimal characters.
    # This character set cannot contain path separators, dots, or any traversal
    # sequences. Only known-format log IDs reach the file operation.
    if [[ ! "$log_id" =~ ^[0-9a-f]{8}$ ]]; then
        echo "Invalid log ID" >&2; return 1
    fi
    cat "/var/app/logs/${log_id}.log"  # vuln-code-snippet safe-line pathtraver_log_id_validated
}
# vuln-code-snippet end pathtraver_log_id_validated

# vuln-code-snippet start pathtraver_realpath_prefix_check
read_user_report() {
    local user_path="$1"
    # realpath -e resolves all symlinks and canonicalizes the path. The prefix
    # check on the resolved path ensures that even if user_path contains ../
    # sequences or symlinks, the final resolved path must be within
    # /var/app/reports/. Traversal is impossible after canonicalization.
    local canonical
    canonical=$(realpath -e "$user_path" 2>/dev/null) || { echo "Path not found" >&2; return 1; }
    if [[ "$canonical" != /var/app/reports/* ]]; then
        echo "Path outside allowed directory" >&2; return 1
    fi
    cat "$canonical"  # vuln-code-snippet safe-line pathtraver_realpath_prefix_check
}
# vuln-code-snippet end pathtraver_realpath_prefix_check

# vuln-code-snippet start pathtraver_basename_display_only
display_filename_label() {
    local user_file="$1"
    # basename strips all directory components from user_file, returning only the
    # final filename component. The result is used only for display — no file
    # operation is performed on user_file or label. No traversal risk exists.
    local label
    label=$(basename "$user_file")  # vuln-code-snippet safe-line pathtraver_basename_display_only
    echo "Processing: ${label}"
}
# vuln-code-snippet end pathtraver_basename_display_only

# vuln-code-snippet start pathtraver_integer_version_path
install_versioned_artifact() {
    local version="$1"
    local artifact_src="$2"
    # version is validated as numeric-only (^[0-9]+$). Digits cannot form a path
    # traversal sequence. The installation path /var/app/releases/VERSION/artifact
    # is safe — only the version number varies, and it is constrained to integers.
    if [[ ! "$version" =~ ^[0-9]+$ ]]; then
        echo "Version must be numeric" >&2; return 1
    fi
    install -D "$artifact_src" "/var/app/releases/${version}/artifact"  # vuln-code-snippet safe-line pathtraver_integer_version_path
}
# vuln-code-snippet end pathtraver_integer_version_path

# vuln-code-snippet start pathtraver_find_basename_only
locate_config_file() {
    local user_input="$1"
    # basename strips directory components from user_input — traversal sequences
    # in the path portion are discarded. find is restricted to /var/app at
    # maxdepth 1. The attacker can only search for filenames within one directory
    # level of /var/app.
    find /var/app -maxdepth 1 -name "$(basename "$user_input")" -type f  # vuln-code-snippet safe-line pathtraver_find_basename_only
}
# vuln-code-snippet end pathtraver_find_basename_only

# vuln-code-snippet start pathtraver_readlink_prefix
read_validated_path() {
    local path="$1"
    # readlink -f resolves the full canonical path including all symlinks. The
    # prefix check on the resolved path ensures that traversal and symlink attacks
    # cannot escape /safe/prefix/. Only then is the file read.
    local resolved
    resolved=$(readlink -f "$path")
    if [[ "$resolved" != /safe/prefix/* ]]; then
        echo "Invalid path" >&2; return 1
    fi
    cat "$resolved"  # vuln-code-snippet safe-line pathtraver_readlink_prefix
}
# vuln-code-snippet end pathtraver_readlink_prefix

# vuln-code-snippet start pathtraver_config_name_validated
load_config_by_name() {
    local config_name="$1"
    # The regex ^[a-zA-Z0-9_-]+$ permits only alphanumerics, underscores, and
    # hyphens. Path separators (/) and dots (.) are excluded, preventing ../
    # traversal. Only valid config names reach the file read.
    if [[ ! "$config_name" =~ ^[a-zA-Z0-9_-]+$ ]]; then
        echo "Invalid config name" >&2; return 1
    fi
    cat "/etc/app/configs/${config_name}.json"  # vuln-code-snippet safe-line pathtraver_config_name_validated
}
# vuln-code-snippet end pathtraver_config_name_validated

# vuln-code-snippet start pathtraver_proc_pid_access
get_process_status() {
    local user_pid="$1"
    # Discrimination test: user controls the PID number, but /proc/$pid/status
    # with a numeric PID is not a path traversal vulnerability. Digits cannot form
    # traversal sequences. The file path structure /proc/NUM/status always resolves
    # within /proc. A numeric PID can only read process status of existing or
    # non-existing processes — not arbitrary filesystem paths.
    if [[ ! "$user_pid" =~ ^[0-9]+$ ]]; then
        echo "Invalid PID" >&2; return 1
    fi
    cat "/proc/${user_pid}/status"  # vuln-code-snippet safe-line pathtraver_proc_pid_access
}
# vuln-code-snippet end pathtraver_proc_pid_access

# vuln-code-snippet start pathtraver_sanitized_filename
save_upload() {
    local raw_filename="$1"
    local content="$2"
    # tr -dc 'A-Za-z0-9._-' strips all characters except alphanumerics, dots,
    # underscores, and hyphens. The forward slash (/) is removed, preventing
    # directory traversal. The resulting filename can only name a file within
    # /tmp/staging/.
    local filename
    filename=$(echo "$raw_filename" | tr -dc 'A-Za-z0-9._-')
    echo "$content" > "/tmp/staging/${filename}"  # vuln-code-snippet safe-line pathtraver_sanitized_filename
}
# vuln-code-snippet end pathtraver_sanitized_filename

# vuln-code-snippet start pathtraver_realpath_relative_base
resolve_relative_path() {
    local user_path="$1"
    # realpath --relative-base=/var/data returns a relative path only if user_path
    # is under /var/data; if it escapes, realpath returns the absolute path (which
    # starts with /), causing the subsequent concatenation to fail or resolve
    # incorrectly as a safeguard.
    local resolved
    resolved=$(realpath --relative-base=/var/data "$user_path")
    cp "$user_path" "/var/data/${resolved}"  # vuln-code-snippet safe-line pathtraver_realpath_relative_base
}
# vuln-code-snippet end pathtraver_realpath_relative_base

# vuln-code-snippet start pathtraver_tar_safe_links
sync_app_content() {
    # --safe-links causes rsync to ignore symlinks that point outside the transfer
    # tree. This prevents symlink-based path traversal attacks during the sync,
    # where a malicious symlink in /var/app/src/ could otherwise cause rsync to
    # overwrite files outside /var/app/dst/.
    rsync -av --safe-links /var/app/src/ /var/app/dst/  # vuln-code-snippet safe-line pathtraver_tar_safe_links
}
# vuln-code-snippet end pathtraver_tar_safe_links

# vuln-code-snippet start pathtraver_zip_junk_paths
archive_with_no_paths() {
    local user_file="$1"
    # The -j (junk paths) flag strips all directory information from archive
    # entries. The archived file is stored with only its basename. Even if
    # user_file is in a sensitive location, the content is archived but the
    # original path is not preserved in the archive.
    zip -j /tmp/archive.zip "$user_file"  # vuln-code-snippet safe-line pathtraver_zip_junk_paths
}
# vuln-code-snippet end pathtraver_zip_junk_paths

# vuln-code-snippet start pathtraver_version_numeric_diff
diff_config_versions() {
    local version="$1"
    # version is validated as numeric-only. The path /var/app/vNUM/config contains
    # only digits in the user-controlled component, preventing ../ traversal. Only
    # integer version numbers can reach the diff operation.
    if [[ ! "$version" =~ ^[0-9]+$ ]]; then
        echo "Version must be numeric" >&2; return 1
    fi
    diff "/var/app/v${version}/config" /etc/app/config  # vuln-code-snippet safe-line pathtraver_version_numeric_diff
}
# vuln-code-snippet end pathtraver_version_numeric_diff

# vuln-code-snippet start pathtraver_service_allowlist_path
read_service_status() {
    local service="$1"
    # The case statement restricts service to four known values. None of these
    # contain path separators or traversal sequences. An attacker cannot inject
    # ../ through a validated case allowlist.
    case "$service" in
        web|api|worker|scheduler)
            cat "/var/app/${service}/status"  # vuln-code-snippet safe-line pathtraver_service_allowlist_path
            ;;
        *)
            echo "Unknown service" >&2; return 1
            ;;
    esac
}
# vuln-code-snippet end pathtraver_service_allowlist_path

# vuln-code-snippet start pathtraver_tar_no_overwrite
extract_archive_safe() {
    local archive="$1"
    # -C /tmp/staging/ confines extraction to the staging directory.
    # --no-overwrite-dir prevents tar from overwriting existing directories
    # (protecting symlink attacks on the target directory itself). While tar could
    # still extract ../ entries, the combination provides meaningful defense in the
    # staging context.
    tar xf "$archive" -C /tmp/staging/ --no-overwrite-dir  # vuln-code-snippet safe-line pathtraver_tar_no_overwrite
}
# vuln-code-snippet end pathtraver_tar_no_overwrite

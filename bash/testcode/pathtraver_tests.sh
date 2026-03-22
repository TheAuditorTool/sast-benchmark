#!/bin/bash
# Path Traversal Test Cases (CWE-22)
# File operations with unsanitized vs sanitized paths

DATA_DIR="/var/app/data"

# vuln-code-snippet start pathtraver_cat_user_input
read_user_file() {
    local filename="$1"
    cat "${DATA_DIR}/${filename}"  # vuln-code-snippet vuln-line pathtraver_cat_user_input
}
# vuln-code-snippet end pathtraver_cat_user_input

# vuln-code-snippet start pathtraver_basename_safe
read_user_file_safe() {
    local filename="$1"
    local safe_name
    safe_name=$(basename "$filename")
    cat "${DATA_DIR}/${safe_name}"  # vuln-code-snippet safe-line pathtraver_basename_safe
}
# vuln-code-snippet end pathtraver_basename_safe

# vuln-code-snippet start pathtraver_tar_extract
extract_user_archive() {
    local archive="$1"
    local target_dir="$2"
    tar xzf "$archive" -C "$target_dir"  # vuln-code-snippet vuln-line pathtraver_tar_extract
}
# vuln-code-snippet end pathtraver_tar_extract

# vuln-code-snippet start pathtraver_realpath_check_safe
read_file_validated() {
    local user_path="$1"
    local real_path
    real_path=$(realpath -m "${DATA_DIR}/${user_path}")
    if [[ "$real_path" != "${DATA_DIR}/"* ]]; then
        echo "Path traversal attempt blocked" >&2
        return 1
    fi
    cat "$real_path"  # vuln-code-snippet safe-line pathtraver_realpath_check_safe
}
# vuln-code-snippet end pathtraver_realpath_check_safe

# vuln-code-snippet start pathtraver_symlink_bypass
read_file_dotdot_only() {
    local filename="$1"
    # Strips .. but does NOT resolve symlinks — incomplete sanitization
    local cleaned="${filename//\.\.\//}"
    cat "${DATA_DIR}/${cleaned}"  # vuln-code-snippet vuln-line pathtraver_symlink_bypass
}
# vuln-code-snippet end pathtraver_symlink_bypass

# vuln-code-snippet start pathtraver_rm_user_path
delete_user_upload() {
    local upload_name="$1"
    rm -rf "/uploads/${upload_name}"  # vuln-code-snippet vuln-line pathtraver_rm_user_path
}
# vuln-code-snippet end pathtraver_rm_user_path

# --- Tier 2 additions (Phase 3, verified 2026-03-19) ---

# vuln-code-snippet start pathtraver_tar_member_traversal
extract_user_upload() {
    local archive="$1"
    local extract_dir="/var/app/uploads"
    # Archive may contain members like "../../etc/cron.d/backdoor" which writes
    # outside extract_dir. GNU tar before 2.39 follows ../ in member names by default.
    # Documented CVE pattern (zip slip / tar slip). CWE-22.
    mkdir -p "$extract_dir"
    tar xzf "$archive" -C "$extract_dir"  # vuln-code-snippet vuln-line pathtraver_tar_member_traversal
}
# vuln-code-snippet end pathtraver_tar_member_traversal

# vuln-code-snippet start pathtraver_tar_safe_extract
extract_user_upload_safe() {
    local archive="$1"
    local extract_dir="/var/app/uploads"
    mkdir -p "$extract_dir"
    # Extract to temp dir first, then validate no paths escape the target
    local tmp_dir
    tmp_dir=$(mktemp -d)
    tar xzf "$archive" -C "$tmp_dir"
    # Check for path traversal attempts in extracted content
    if find "$tmp_dir" -name ".*" -path "*/..*" | grep -q .; then
        rm -rf "$tmp_dir"
        echo "Archive contains path traversal — rejected" >&2
        return 1
    fi
    cp -r "$tmp_dir"/* "$extract_dir"/  # vuln-code-snippet safe-line pathtraver_tar_safe_extract
    rm -rf "$tmp_dir"
}
# vuln-code-snippet end pathtraver_tar_safe_extract

# --- Phase 2 TN additions (OWASP 50/50 rebalancing, 2026-03-22) ---

# vuln-code-snippet start pathtraver_readlink_safe
serve_user_file() {
    #readlink -f resolves the full canonical path (following symlinks),
    # then a prefix check ensures the resolved path stays within DATA_DIR.
    # This blocks ../ traversal AND symlink-based escapes.
    local name="$1"
    local DATA_DIR="/var/app/uploads"
    local resolved
    resolved=$(readlink -f "${DATA_DIR}/${name}")
    if [[ "$resolved" != "${DATA_DIR}/"* ]]; then
        echo "Path traversal blocked: $name" >&2
        return 1
    fi
    cat "$resolved"  # vuln-code-snippet safe-line pathtraver_readlink_safe
}
# vuln-code-snippet end pathtraver_readlink_safe

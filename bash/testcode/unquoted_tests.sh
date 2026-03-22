#!/bin/bash
# Unquoted Expansion Test Cases (CWE-78 variant)
# Word splitting and globbing risks

# vuln-code-snippet start unquoted_for_loop
process_files() {
    local files="$1"
    for f in $files; do  # vuln-code-snippet vuln-line unquoted_for_loop
        echo "Processing: $f"
    done
}
# vuln-code-snippet end unquoted_for_loop

# vuln-code-snippet start unquoted_quoted_array_safe
process_files_safe() {
    local -a files=("$@")
    for f in "${files[@]}"; do  # vuln-code-snippet safe-line unquoted_quoted_array_safe
        echo "Processing: $f"
    done
}
# vuln-code-snippet end unquoted_quoted_array_safe

# vuln-code-snippet start unquoted_test_bracket
check_file_exists() {
    local path="$1"
    if [ -f $path ]; then  # vuln-code-snippet vuln-line unquoted_test_bracket
        echo "exists"
    fi
}
# vuln-code-snippet end unquoted_test_bracket

# vuln-code-snippet start unquoted_double_bracket_safe
check_file_exists_safe() {
    local path="$1"
    if [[ -f "$path" ]]; then  # vuln-code-snippet safe-line unquoted_double_bracket_safe
        echo "exists"
    fi
}
# vuln-code-snippet end unquoted_double_bracket_safe

# vuln-code-snippet start unquoted_rm_variable
cleanup_directory() {
    local dir="$1"
    rm -rf $dir  # vuln-code-snippet vuln-line unquoted_rm_variable
}
# vuln-code-snippet end unquoted_rm_variable

# vuln-code-snippet start unquoted_rm_quoted_safe
cleanup_directory_safe() {
    local dir="$1"
    rm -rf "$dir"  # vuln-code-snippet safe-line unquoted_rm_quoted_safe
}
# vuln-code-snippet end unquoted_rm_quoted_safe

# --- Phase 2 TN additions (OWASP 50/50 rebalancing, 2026-03-22) ---

# vuln-code-snippet start unquoted_array_iteration_safe
deploy_services() {
    # Safe: array expansion "${services[@]}" is properly double-quoted.
    # Each array element is preserved as a single token, even if it contains spaces.
    local -a services=("$@")
    local svc
    for svc in "${services[@]}"; do
        systemctl restart "$svc"  # vuln-code-snippet safe-line unquoted_array_iteration_safe
    done
}
# vuln-code-snippet end unquoted_array_iteration_safe

# vuln-code-snippet start unquoted_cp_both_quoted_safe
archive_logs() {
    # Safe: both source and destination paths are properly double-quoted.
    # Without quotes, paths with spaces would split into separate arguments.
    local log_dir="$1"
    local archive_dir="$2"
    cp -r "$log_dir" "$archive_dir"  # vuln-code-snippet safe-line unquoted_cp_both_quoted_safe
}
# vuln-code-snippet end unquoted_cp_both_quoted_safe

# vuln-code-snippet start unquoted_read_r_ifs_safe
parse_env_file() {
    # Safe: read -r prevents backslash interpretation, IFS='=' splits only
    # on equals sign. Variables are properly quoted in the export statement.
    local file="$1"
    local key value
    while IFS='=' read -r key value; do
        export "$key=$value"  # vuln-code-snippet safe-line unquoted_read_r_ifs_safe
    done < "$file"
}
# vuln-code-snippet end unquoted_read_r_ifs_safe

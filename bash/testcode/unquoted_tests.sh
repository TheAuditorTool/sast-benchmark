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

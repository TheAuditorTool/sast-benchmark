#!/bin/bash
# Unquoted Variable Expansion Test Cases (CWE-78)
# Word splitting and glob expansion via unquoted variables enable OS command injection.

# --- True Positives ---

# vuln-code-snippet start unquoted_grep_pattern
search_logs() {
    local pattern="$1"
    local logfile="$2"
    # Unquoted $pattern undergoes glob expansion before grep sees it; a space in
    # $pattern splits it into multiple arguments, and a leading dash enables grep flags.
    # shellcheck disable=SC2086
    grep $pattern $logfile  # vuln-code-snippet vuln-line unquoted_grep_pattern
}
# vuln-code-snippet end unquoted_grep_pattern

# vuln-code-snippet start unquoted_scp_paths
transfer_artifact() {
    local src="$1"
    local dst="$2"
    # Word splitting on $src and $dst turns a path with spaces into multiple
    # positional arguments, breaking the intended single-source single-dest transfer.
    # shellcheck disable=SC2086
    scp $src user@host:$dst  # vuln-code-snippet vuln-line unquoted_scp_paths
}
# vuln-code-snippet end unquoted_scp_paths

# vuln-code-snippet start unquoted_ssh_host_cmd
run_remote_command() {
    local host="$1"
    local cmd="$2"
    # Word splitting on $cmd allows injection of additional SSH options or commands
    # if the caller embeds spaces; e.g. "hostname -o StrictHostKeyChecking=no".
    # shellcheck disable=SC2086
    ssh $host $cmd  # vuln-code-snippet vuln-line unquoted_ssh_host_cmd
}
# vuln-code-snippet end unquoted_ssh_host_cmd

# vuln-code-snippet start unquoted_chmod_args
set_permissions() {
    local perms="$1"
    local file="$2"
    # Unquoted $file: a filename containing spaces becomes multiple chmod targets.
    # Unquoted $perms: a value like "644 /etc/shadow" gains an extra argument.
    # shellcheck disable=SC2086
    chmod $perms $file  # vuln-code-snippet vuln-line unquoted_chmod_args
}
# vuln-code-snippet end unquoted_chmod_args

# vuln-code-snippet start unquoted_mkdir_name
create_workspace() {
    local dir_name="$1"
    # Unquoted $dir_name: spaces cause mkdir to create several separate directories
    # rather than one directory whose name contains spaces.
    # shellcheck disable=SC2086
    mkdir $dir_name  # vuln-code-snippet vuln-line unquoted_mkdir_name
}
# vuln-code-snippet end unquoted_mkdir_name

# vuln-code-snippet start unquoted_mv_paths
relocate_file() {
    local src="$1"
    local dst="$2"
    # Both arguments unquoted; a space anywhere in the paths causes argument
    # splitting and can redirect the move to an unintended location.
    # shellcheck disable=SC2086
    mv $src $dst  # vuln-code-snippet vuln-line unquoted_mv_paths
}
# vuln-code-snippet end unquoted_mv_paths

# vuln-code-snippet start unquoted_single_bracket_glob
check_extension() {
    local var="$1"
    # Inside single [ ], $var is unquoted so word-splitting applies and the
    # glob *.sh expands to matching filenames in the current directory before
    # the comparison is evaluated.
    # shellcheck disable=SC2086,SC2081
    [ $var == *.sh ]  # vuln-code-snippet vuln-line unquoted_single_bracket_glob
}
# vuln-code-snippet end unquoted_single_bracket_glob

# vuln-code-snippet start unquoted_find_both_args
locate_scripts() {
    local searchdir="$1"
    local pattern="$2"
    # Unquoted $searchdir: a path with spaces splits into multiple find roots.
    # Unquoted $pattern: glob chars in $pattern expand before find sees them.
    # shellcheck disable=SC2086
    find $searchdir -name $pattern  # vuln-code-snippet vuln-line unquoted_find_both_args
}
# vuln-code-snippet end unquoted_find_both_args

# vuln-code-snippet start unquoted_wc_filename
count_lines() {
    local user_file="$1"
    # Unquoted $user_file: spaces in the path create extra wc arguments, potentially
    # counting lines across unintended files or injecting flags like --files0-from.
    # shellcheck disable=SC2086
    wc -l $user_file  # vuln-code-snippet vuln-line unquoted_wc_filename
}
# vuln-code-snippet end unquoted_wc_filename

# vuln-code-snippet start unquoted_sort_io_files
sort_report() {
    local input_file="$1"
    local output_file="$2"
    # $input_file is unquoted; word splitting turns "my file.txt" into two
    # separate sort arguments. $output_file appears after > so the shell handles
    # it, but the redirect itself may mis-target if the variable contains spaces.
    # shellcheck disable=SC2086
    sort $input_file > "$output_file"  # vuln-code-snippet vuln-line unquoted_sort_io_files
}
# vuln-code-snippet end unquoted_sort_io_files

# vuln-code-snippet start unquoted_for_list_rm
purge_temp_files() {
    # Unquoted $FILE_LIST undergoes word splitting, so the list is determined by
    # IFS, not by intended delimiters. Filenames with spaces become separate tokens.
    # Unquoted $file inside rm receives those split tokens individually.
    # shellcheck disable=SC2086
    for file in $FILE_LIST; do  # vuln-code-snippet vuln-line unquoted_for_list_rm
        # shellcheck disable=SC2086
        rm $file
    done
}
# vuln-code-snippet end unquoted_for_list_rm

# vuln-code-snippet start unquoted_cd_path
navigate_to_user_dir() {
    local user_path
    read -r user_path
    # Unquoted $user_path in cd: bash interprets spaces as argument separators,
    # yielding "cd: too many arguments" or navigating to only the first word.
    # shellcheck disable=SC2086,SC2164
    cd $user_path  # vuln-code-snippet vuln-line unquoted_cd_path
}
# vuln-code-snippet end unquoted_cd_path

# vuln-code-snippet start unquoted_printf_format
emit_message() {
    local format="$1"
    local value="$2"
    # Unquoted $format: attacker-controlled format directives (%s, %b, %q, %n on
    # some platforms) are interpreted literally by printf, enabling format-string
    # style data leakage or unexpected output behaviour.
    # shellcheck disable=SC2086,SC2059
    printf $format "$value"  # vuln-code-snippet vuln-line unquoted_printf_format
}
# vuln-code-snippet end unquoted_printf_format

# vuln-code-snippet start unquoted_eval_double
expand_template() {
    local user_input="$1"
    # eval with unquoted $user_input causes double evaluation: the shell first
    # splits and expands $user_input, then eval re-interprets the result as a
    # shell command. Any shell metacharacter in $user_input is executed.
    # shellcheck disable=SC2086
    eval echo $user_input  # vuln-code-snippet vuln-line unquoted_eval_double
}
# vuln-code-snippet end unquoted_eval_double

# vuln-code-snippet start unquoted_test_and_source
apply_config() {
    local config="$1"
    # Unquoted $config in [ -f ]: spaces or glob chars cause word splitting before
    # the test, potentially matching unintended paths. Unquoted $config in source
    # has the same splitting effect, loading a different file than intended.
    # shellcheck disable=SC2086,SC1090
    [ -f $config ] && source $config  # vuln-code-snippet vuln-line unquoted_test_and_source
}
# vuln-code-snippet end unquoted_test_and_source

# --- True Negatives ---

# vuln-code-snippet start unquoted_grep_quoted
search_logs_quoted() {
    local pattern="$1"
    local logfile="$2"
    grep "$pattern" "$logfile"  # vuln-code-snippet safe-line unquoted_grep_quoted
}
# vuln-code-snippet end unquoted_grep_quoted

# vuln-code-snippet start unquoted_scp_quoted
transfer_artifact_quoted() {
    local src="$1"
    local dst="$2"
    scp "$src" "user@host:$dst"  # vuln-code-snippet safe-line unquoted_scp_quoted
}
# vuln-code-snippet end unquoted_scp_quoted

# vuln-code-snippet start unquoted_ssh_quoted
run_remote_command_quoted() {
    local host="$1"
    local cmd="$2"
    # shellcheck disable=SC2029
    ssh "$host" "$cmd"  # vuln-code-snippet safe-line unquoted_ssh_quoted
}
# vuln-code-snippet end unquoted_ssh_quoted

# vuln-code-snippet start unquoted_chmod_quoted
set_permissions_quoted() {
    local perms="$1"
    local file="$2"
    chmod "$perms" "$file"  # vuln-code-snippet safe-line unquoted_chmod_quoted
}
# vuln-code-snippet end unquoted_chmod_quoted

# vuln-code-snippet start unquoted_mkdir_quoted
create_workspace_quoted() {
    local dir_name="$1"
    mkdir "$dir_name"  # vuln-code-snippet safe-line unquoted_mkdir_quoted
}
# vuln-code-snippet end unquoted_mkdir_quoted

# vuln-code-snippet start unquoted_mv_quoted
relocate_file_quoted() {
    local src="$1"
    local dst="$2"
    mv "$src" "$dst"  # vuln-code-snippet safe-line unquoted_mv_quoted
}
# vuln-code-snippet end unquoted_mv_quoted

# vuln-code-snippet start unquoted_single_bracket_quoted
check_extension_quoted() {
    local var="$1"
    # Double-quoting $var inside [ ] prevents word splitting and glob expansion;
    # the comparison is against the literal string "expected", not a glob pattern.
    [ "$var" = "expected" ]  # vuln-code-snippet safe-line unquoted_single_bracket_quoted
}
# vuln-code-snippet end unquoted_single_bracket_quoted

# vuln-code-snippet start unquoted_find_quoted
locate_scripts_quoted() {
    local searchdir="$1"
    local pattern="$2"
    find "$searchdir" -name "$pattern"  # vuln-code-snippet safe-line unquoted_find_quoted
}
# vuln-code-snippet end unquoted_find_quoted

# vuln-code-snippet start unquoted_wc_quoted
count_lines_quoted() {
    local user_file="$1"
    wc -l "$user_file"  # vuln-code-snippet safe-line unquoted_wc_quoted
}
# vuln-code-snippet end unquoted_wc_quoted

# vuln-code-snippet start unquoted_sort_quoted
sort_report_quoted() {
    local input_file="$1"
    local output_file="$2"
    sort "$input_file" > "$output_file"  # vuln-code-snippet safe-line unquoted_sort_quoted
}
# vuln-code-snippet end unquoted_sort_quoted

# vuln-code-snippet start unquoted_for_array_quoted
purge_temp_files_quoted() {
    # Array expansion with "${FILE_LIST[@]}" preserves elements that contain
    # spaces as single tokens; IFS-based splitting does not apply to array refs.
    for file in "${FILE_LIST[@]}"; do
        rm "$file"  # vuln-code-snippet safe-line unquoted_for_array_quoted
    done
}
# vuln-code-snippet end unquoted_for_array_quoted

# vuln-code-snippet start unquoted_cd_quoted
navigate_to_user_dir_quoted() {
    local user_path
    read -r user_path
    # shellcheck disable=SC2164
    cd "$user_path"  # vuln-code-snippet safe-line unquoted_cd_quoted
}
# vuln-code-snippet end unquoted_cd_quoted

# vuln-code-snippet start unquoted_printf_literal
emit_message_literal() {
    local value="$1"
    # Single-quoted format string is a fixed literal; printf never sees
    # attacker-controlled format directives. $value is passed as a data argument.
    printf '%s\n' "$value"  # vuln-code-snippet safe-line unquoted_printf_literal
}
# vuln-code-snippet end unquoted_printf_literal

# vuln-code-snippet start unquoted_double_bracket_source
apply_config_quoted() {
    local config="$1"
    # [[ ]] never performs word splitting or glob expansion on its operands,
    # so $config is safe even unquoted inside [[. The source argument is
    # quoted for defense in depth against path-with-spaces issues.
    # shellcheck disable=SC1090
    [[ -f "$config" ]] && source "$config"  # vuln-code-snippet safe-line unquoted_double_bracket_source
}
# vuln-code-snippet end unquoted_double_bracket_source

# vuln-code-snippet start unquoted_positional_reset
normalize_args() {
    # "set -- $*" (unquoted) is the risky form: $* undergoes word splitting so
    # arguments with spaces are re-split. "$@" preserves each positional parameter
    # as a separate word regardless of IFS, making this form safe.
    set -- "$@"  # vuln-code-snippet safe-line unquoted_positional_reset
    echo "Arg count: $#"
}
# vuln-code-snippet end unquoted_positional_reset

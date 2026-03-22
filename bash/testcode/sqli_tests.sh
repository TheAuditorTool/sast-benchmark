#!/bin/bash
# SQL Injection Test Cases (CWE-89)
# Shell-based SQL clients with string interpolation patterns

DB_FILE="/tmp/test.db"

# vuln-code-snippet start sqli_direct_interpolation
get_user_by_name() {
    local username="$1"
    sqlite3 "$DB_FILE" "SELECT * FROM users WHERE name = '${username}'"  # vuln-code-snippet vuln-line sqli_direct_interpolation
}
# vuln-code-snippet end sqli_direct_interpolation

# vuln-code-snippet start sqli_no_user_input_safe
get_total_users() {
    sqlite3 "$DB_FILE" "SELECT COUNT(*) FROM users"  # vuln-code-snippet safe-line sqli_no_user_input_safe
}
# vuln-code-snippet end sqli_no_user_input_safe

# vuln-code-snippet start sqli_multihop_taint
_sanitize_name() {
    local raw="$1"
    # Pretends to sanitize but just trims whitespace
    echo "$raw" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//'
}

_build_query() {
    local name="$1"
    echo "SELECT email FROM users WHERE name = '${name}'"
}

lookup_user_email() {
    local input="$1"
    local cleaned
    cleaned=$(_sanitize_name "$input")
    local query
    query=$(_build_query "$cleaned")
    sqlite3 "$DB_FILE" "$query"  # vuln-code-snippet vuln-line sqli_multihop_taint
}
# vuln-code-snippet end sqli_multihop_taint

# vuln-code-snippet start sqli_validated_integer_safe
get_user_by_id() {
    local id="$1"
    if [[ ! "$id" =~ ^[0-9]+$ ]]; then
        echo "Invalid ID" >&2
        return 1
    fi
    sqlite3 "$DB_FILE" "SELECT * FROM users WHERE id = ${id}"  # vuln-code-snippet safe-line sqli_validated_integer_safe
}
# vuln-code-snippet end sqli_validated_integer_safe

# vuln-code-snippet start sqli_mysql_cli
insert_log_entry() {
    local message="$1"
    local level="$2"
    mysql -u app -pdbpass mydb -e "INSERT INTO logs (message, level) VALUES ('${message}', '${level}')"  # vuln-code-snippet vuln-line sqli_mysql_cli
}
# vuln-code-snippet end sqli_mysql_cli

# vuln-code-snippet start sqli_table_name_injection
export_table() {
    local table_name="$1"
    local output_file="$2"
    sqlite3 "$DB_FILE" "SELECT * FROM ${table_name}" > "$output_file"  # vuln-code-snippet vuln-line sqli_table_name_injection
}
# vuln-code-snippet end sqli_table_name_injection

# --- Tier 2 additions (Phase 3, verified 2026-03-19) ---

# vuln-code-snippet start sqli_order_by_injection
list_users_sorted() {
    local sort_column="$1"
    # ORDER BY clause can't be parameterized in SQL. User controls column position.
    # Attack: sort_column="1; DROP TABLE users--" executes destructive SQL.
    sqlite3 "$DB_FILE" "SELECT name, email FROM users ORDER BY ${sort_column}"  # vuln-code-snippet vuln-line sqli_order_by_injection
}
# vuln-code-snippet end sqli_order_by_injection

# vuln-code-snippet start sqli_order_by_whitelist_safe
list_users_sorted_safe() {
    local sort_column="$1"
    # Whitelist validation: only allow known column names
    case "$sort_column" in
        name|email|created_at|id)
            sqlite3 "$DB_FILE" "SELECT name, email FROM users ORDER BY ${sort_column}"  # vuln-code-snippet safe-line sqli_order_by_whitelist_safe
            ;;
        *)
            echo "Invalid sort column: $sort_column" >&2
            return 1
            ;;
    esac
}
# vuln-code-snippet end sqli_order_by_whitelist_safe

# --- Phase 2 TN additions (OWASP 50/50 rebalancing, 2026-03-22) ---

# vuln-code-snippet start sqli_printf_q_escaped_safe
search_deployments_safe() {
    #user input is escaped with printf %q before SQL interpolation.
    # printf %q escapes all shell metacharacters and single quotes,
    # preventing SQL injection when the value is wrapped in SQL string quotes.
    local search_val="$1"
    local escaped
    escaped=$(printf '%q' "$search_val")
    sqlite3 "$DB_FILE" "SELECT * FROM deployments WHERE name = '${escaped}'"  # vuln-code-snippet safe-line sqli_printf_q_escaped_safe
}
# vuln-code-snippet end sqli_printf_q_escaped_safe

# vuln-code-snippet start sqli_where_constant_safe
get_active_deployments() {
    #entirely static query with no user-provided values.
    # All literals ('active', 10) are hardcoded constants.
    sqlite3 "$DB_FILE" "SELECT * FROM deployments WHERE status = 'active' ORDER BY created_at DESC LIMIT 10"  # vuln-code-snippet safe-line sqli_where_constant_safe
}
# vuln-code-snippet end sqli_where_constant_safe

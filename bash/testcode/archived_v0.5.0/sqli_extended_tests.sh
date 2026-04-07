#!/bin/bash
# SQL Injection Extended Test Cases (CWE-89)
# Additional patterns beyond v0.4.0: PostgreSQL CLI, INSERT/UPDATE clauses,
# LIKE pattern injection, and extended safe variants.

DB_FILE="/tmp/test.db"

# vuln-code-snippet start sqli_psql_interpolation
query_users_by_email() {
    local email="$1"
    # psql -c executes the argument as SQL. String interpolation allows ' OR '1'='1
    # to bypass the WHERE clause and return all rows.
    psql -d appdb -c "SELECT * FROM users WHERE email='${email}'"  # vuln-code-snippet vuln-line sqli_psql_interpolation
}
# vuln-code-snippet end sqli_psql_interpolation

# vuln-code-snippet start sqli_insert_values
log_audit_event() {
    local msg="$1"
    local level="$2"
    # Both VALUES fields are string-interpolated. Injecting ', 'x'); DROP TABLE audit_log;--
    # into msg terminates the string literal and appends arbitrary SQL.
    sqlite3 "$DB_FILE" "INSERT INTO audit_log (message, level) VALUES ('${msg}', '${level}')"  # vuln-code-snippet vuln-line sqli_insert_values
}
# vuln-code-snippet end sqli_insert_values

# vuln-code-snippet start sqli_update_set
update_user_role() {
    local new_role="$1"
    local uid="$2"
    # Both SET value and WHERE clause are string-interpolated. A crafted new_role value
    # terminates the string and can modify all rows by injecting ' WHERE '1'='1.
    sqlite3 "$DB_FILE" "UPDATE users SET role='${new_role}' WHERE id='${uid}'"  # vuln-code-snippet vuln-line sqli_update_set
}
# vuln-code-snippet end sqli_update_set

# vuln-code-snippet start sqli_like_pattern
search_users() {
    local search="$1"
    # The LIKE pattern is string-interpolated. Injecting %' OR '1'='1 OR name LIKE '%
    # breaks out of the LIKE pattern and returns all rows.
    sqlite3 "$DB_FILE" "SELECT id, name, email FROM users WHERE name LIKE '%${search}%'"  # vuln-code-snippet vuln-line sqli_like_pattern
}
# vuln-code-snippet end sqli_like_pattern

# vuln-code-snippet start sqli_integer_coercion
get_user_by_id_arith() {
    local id="$1"
    # Bash arithmetic expansion $(( id + 0 )) evaluates the operand as an integer.
    # If id contains non-numeric characters, bash raises a syntax error before the
    # sqlite3 call executes. Only pure integers reach the SQL engine, eliminating injection.
    sqlite3 "$DB_FILE" "SELECT * FROM users WHERE id = $(( id + 0 ))"  # vuln-code-snippet safe-line sqli_integer_coercion
}
# vuln-code-snippet end sqli_integer_coercion

# vuln-code-snippet start sqli_static_only
count_active_sessions() {
    # This is a discrimination test — no user input anywhere in the query. Tools that
    # flag all sqlite3 calls without taint analysis will produce a false positive here.
    sqlite3 "$DB_FILE" "SELECT COUNT(*) FROM active_sessions WHERE expires_at > strftime('%s','now')"  # vuln-code-snippet safe-line sqli_static_only
}
# vuln-code-snippet end sqli_static_only

# vuln-code-snippet start sqli_quote_doubling
find_user_by_name() {
    local input="$1"
    local escaped="${input//\'/\'\'}"
    # The ${var//old/new} parameter expansion replaces every single-quote with two
    # consecutive single-quotes — the SQL-standard escaping method for string literals.
    # This is the correct escape method for sqlite3 (unlike printf %q which uses
    # backslashes that SQLite does not recognize).
    sqlite3 "$DB_FILE" "SELECT id, email FROM users WHERE name = '${escaped}'"  # vuln-code-snippet safe-line sqli_quote_doubling
}
# vuln-code-snippet end sqli_quote_doubling

# vuln-code-snippet start sqli_column_whitelist_strict
list_users_by_column() {
    local col="$1"
    # Column identifiers cannot be parameterized in SQL. This allowlist approach is the
    # correct mitigation: only four known-safe column names reach the query. Any other
    # input is rejected before the SQL executes.
    case "$col" in
        id|name|email|created_at)
            sqlite3 "$DB_FILE" "SELECT * FROM users ORDER BY ${col}"  # vuln-code-snippet safe-line sqli_column_whitelist_strict
            ;;
        *)
            echo "Invalid column" >&2
            return 1
            ;;
    esac
}
# vuln-code-snippet end sqli_column_whitelist_strict

# vuln-code-snippet start sqli_integer_validated_id
get_user_by_id() {
    local id="$1"
    # Strict integer validation before interpolation. The regex anchors ^[0-9]+$
    # guarantee no shell metacharacters or SQL syntax can survive — the value
    # reaching the query is provably a non-negative integer literal.
    if [[ ! "$id" =~ ^[0-9]+$ ]]; then
        echo "Invalid id" >&2
        return 1
    fi
    sqlite3 "$DB_FILE" "SELECT * FROM users WHERE id = ${id}"  # vuln-code-snippet safe-line sqli_integer_validated_id
}
# vuln-code-snippet end sqli_integer_validated_id

# vuln-code-snippet start sqli_readonly_static_query
get_schema_version() {
    # Fully static query: no user input anywhere in the SQL string.
    # The sqlite3 call reads a single metadata value from a fixed table;
    # there is no interpolation path for injection.
    sqlite3 "$DB_FILE" "SELECT version FROM schema_meta WHERE id = 1"  # vuln-code-snippet safe-line sqli_readonly_static_query
}
# vuln-code-snippet end sqli_readonly_static_query

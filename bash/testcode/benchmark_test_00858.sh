#!/bin/bash
_sanitize_name() {
    local raw="$1"
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
    sqlite3 "$DB_FILE" "$query"
}

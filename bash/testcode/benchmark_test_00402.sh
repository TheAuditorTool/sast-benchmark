#!/bin/bash
update_user_role() {
    local new_role="$1"
    local uid="$2"
    sqlite3 "$DB_FILE" "UPDATE users SET role='${new_role}' WHERE id='${uid}'"
}

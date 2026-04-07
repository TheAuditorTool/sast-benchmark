#!/bin/bash
query_users_by_email() {
    local email="$1"
    psql -d appdb -c "SELECT * FROM users WHERE email='${email}'"
}

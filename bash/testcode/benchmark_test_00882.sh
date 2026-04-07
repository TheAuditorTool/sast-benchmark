#!/bin/bash
find_customer() {
    local phone="$1"
    local query="SELECT id, name FROM customers WHERE phone = '${phone}'"
    mysql -u "$DB_USER" -p"$DB_PASS" crm_db -e "$query"
}

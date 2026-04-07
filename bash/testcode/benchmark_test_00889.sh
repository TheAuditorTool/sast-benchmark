#!/bin/bash
fetch_top_products() {
    local limit="$1"
    if [[ "$limit" =~ ^[0-9]+$ ]]; then
        mysql -u "$DB_USER" -p"$DB_PASS" shop_db -e "SELECT name, sales FROM products ORDER BY sales DESC LIMIT $limit"
    fi
}

#!/bin/bash
fetch_product() {
    local product_name="$1"
    mysql -u root -p"$DB_PASS" -e "SELECT * FROM products WHERE name = '${product_name}'" store_db
}

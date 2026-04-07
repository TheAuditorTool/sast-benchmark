#!/bin/bash
list_product_categories() {
    mysql -u "$DB_USER" -p"$DB_PASS" store_db -e "SELECT DISTINCT category FROM products ORDER BY category"
}

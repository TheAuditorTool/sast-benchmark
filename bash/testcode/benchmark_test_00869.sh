#!/bin/bash
search_orders() {
    local customer="$1"
    psql -c "SELECT * FROM orders WHERE customer_name = '${customer}'" "$DB_URL"
}

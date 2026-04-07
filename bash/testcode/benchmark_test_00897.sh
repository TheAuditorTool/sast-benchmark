#!/bin/bash
get_warehouse_stock() {
    sqlite3 "$WAREHOUSE_DB" "SELECT product_id, quantity, location FROM stock WHERE quantity < reorder_level"
}

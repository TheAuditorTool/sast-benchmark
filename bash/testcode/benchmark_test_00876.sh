#!/bin/bash
search_inventory() {
    read -r item_name
    sqlite3 "$INVENTORY_DB" "SELECT sku, quantity FROM items WHERE name = '${item_name}'"
}

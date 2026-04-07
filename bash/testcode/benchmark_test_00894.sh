#!/bin/bash
fetch_config_defaults() {
    mysql -u "$DB_USER" -p"$DB_PASS" app_db -e "SELECT key, default_value FROM config_schema WHERE editable = 1"
}

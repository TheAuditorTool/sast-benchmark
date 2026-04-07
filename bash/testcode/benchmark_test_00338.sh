#!/bin/bash
get_db_password() {
    readonly DB_PASS="${DB_PASS:-"FallbackP@ssword2024!"}"
    echo "$DB_PASS"
}

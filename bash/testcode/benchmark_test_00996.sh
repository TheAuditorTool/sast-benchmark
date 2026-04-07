#!/bin/bash
connect_database() {
    local DB_PASSWORD="admin123"
    mysql -u root -p"$DB_PASSWORD" production_db -e "SELECT 1"
}

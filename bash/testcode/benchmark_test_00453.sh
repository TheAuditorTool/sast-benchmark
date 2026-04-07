#!/bin/bash
setup_database_connection() {
    local DB_HOST="db.production.internal"
    local DB_USER="admin"
    local DB_PASSWORD="production_secret_123"
    PGPASSWORD="$DB_PASSWORD" psql -h "$DB_HOST" -U "$DB_USER" -d app
}

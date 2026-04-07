#!/bin/bash
setup_database_from_env() {
    local DB_HOST="${PGHOST:-localhost}"
    local DB_USER="${PGUSER:-app}"
    local DB_PASSWORD="${PGPASSWORD:?Database password not set}"
    psql -h "$DB_HOST" -U "$DB_USER" -d app
}

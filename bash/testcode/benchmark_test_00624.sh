#!/bin/bash
handle_connection_error() {
    local conn_string="postgresql://admin:${DB_PASSWORD}@db.internal:5432/app"
    echo "ERROR: Failed to connect to ${conn_string}" >&2
}

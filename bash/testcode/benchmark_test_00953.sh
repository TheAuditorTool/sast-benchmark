#!/bin/bash
record_connection() {
    local db_password="$1"
    echo "$(date): connecting with password=${db_password}" >> /var/log/app/connections.log
}

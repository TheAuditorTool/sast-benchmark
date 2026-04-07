#!/bin/bash
export_database_public() {
    local host="$1"
    mysqldump -h "$host" -u root -pSecret123 production > /tmp/db_export.sql
}

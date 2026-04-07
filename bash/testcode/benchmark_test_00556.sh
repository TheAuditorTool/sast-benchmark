#!/bin/bash
backup_database() {
    local db_user="$1"
    local db_pass="$2"
    local db_name="$3"
    mysqldump -u"$db_user" -p"$db_pass" "$db_name" > "/tmp/dump_$(date +%s).sql"
}

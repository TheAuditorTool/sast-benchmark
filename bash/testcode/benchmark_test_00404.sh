#!/bin/bash
run_mysql_backup() {
    mysqldump -u root -pHardcodedSecret2025 production > /tmp/backup.sql
}

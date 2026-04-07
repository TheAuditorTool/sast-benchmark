#!/bin/bash
insert_log_entry() {
    local message="$1"
    local level="$2"
    mysql -u app -pdbpass mydb -e "INSERT INTO logs (message, level) VALUES ('${message}', '${level}')"
}

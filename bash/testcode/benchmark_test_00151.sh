#!/bin/bash
prompt_for_password() {
    read -rs -p "Database password: " DB_PASS
    echo
    mysql -h db.internal.corp -uapp -p"$DB_PASS" appdb
}

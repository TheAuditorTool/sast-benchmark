#!/bin/bash
connect_db_via_tunnel() {
    local bastion="$1"
    local db_host="$2"
    local pass="$3"
    ssh -f -N -L "3307:${db_host}:3306" "tunnel@${bastion}"
    mysql -h 127.0.0.1 -P 3307 -uapp -p"$pass" appdb
}

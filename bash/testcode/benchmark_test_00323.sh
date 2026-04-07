#!/bin/bash
connect_to_database() {
    local user="$1"
    local pass="$2"
    local host="$3"
    mongosh "mongodb://${user}:${pass}@${host}:27017/appdb"
}

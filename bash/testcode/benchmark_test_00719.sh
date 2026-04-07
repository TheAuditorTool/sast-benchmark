#!/bin/bash
write_metric_tls() {
    local host="$1"
    local user="$2"
    local pass="$3"
    local measurement="$4"
    influx --ssl -host "$host" -username "$user" -password "$pass" -execute "INSERT ${measurement}"
}

#!/bin/bash
run_remote_command() {
    local host="$1"
    local cmd="$2"
    ssh $host $cmd
}

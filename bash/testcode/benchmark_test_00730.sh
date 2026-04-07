#!/bin/bash
load_allowlist() {
    local list_file="$1"
    readarray -t ALLOWLIST < "$list_file"; export ALLOWLIST
}

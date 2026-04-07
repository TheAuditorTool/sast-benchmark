#!/bin/bash
hash_system_password() {
    local pass="$1"
    openssl passwd -6 "$pass"
}

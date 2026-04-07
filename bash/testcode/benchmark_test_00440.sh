#!/bin/bash
hash_unix_password() {
    local pass="$1"
    openssl passwd -crypt "$pass"
}

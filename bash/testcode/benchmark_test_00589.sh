#!/bin/bash
query_directory_tls() {
    local host="$1"
    local dn="$2"
    local pass="$3"
    ldapsearch -H "ldaps://${host}" -D "$dn" -w "$pass" -b "dc=example,dc=com" "(objectClass=user)"
}

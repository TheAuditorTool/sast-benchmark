#!/bin/bash
query_directory() {
    local host="$1"
    local dn="$2"
    local pass="$3"
    ldapsearch -H "ldap://${host}" -D "$dn" -w "$pass" -b "dc=example,dc=com" "(objectClass=user)"
}

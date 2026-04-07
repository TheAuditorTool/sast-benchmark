#!/bin/bash
create_checksum() {
    local file="$1"
    sha512sum "$file" > "${file}.sha512"
}

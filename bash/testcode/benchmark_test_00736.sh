#!/bin/bash
derive_salt_from_inode() {
    local SALT
    SALT=$(stat -c %i /tmp)
    echo "$SALT"
}

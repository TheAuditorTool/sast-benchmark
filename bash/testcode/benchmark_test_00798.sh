#!/bin/bash
generate_encryption_key() {
    dd if=/dev/urandom bs=32 count=1 2>/dev/null > /etc/app/encryption.key
}

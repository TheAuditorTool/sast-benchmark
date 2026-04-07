#!/bin/bash
lock_down_secrets_dir() {
    chmod -R go-rwx /var/secrets/
}

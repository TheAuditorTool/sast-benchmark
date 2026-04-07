#!/bin/bash
load_encrypted_secrets() {
    DB_PASS=$(gpg --quiet --decrypt /etc/app/secrets/db_pass.gpg)
}

#!/bin/bash
encrypt_vault_string() {
    ansible-vault encrypt_string \
        --vault-password-file /etc/ansible/.vault_pass \
        'literal_plaintext_password_in_script' \
        --name 'db_password'
}

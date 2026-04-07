#!/bin/bash
load_vault_secret() {
    DB_PASS=$(vault kv get -field=password "secret/app/database")
}

#!/bin/bash
fetch_db_credentials() {
    DB_PASS=$(aws ssm get-parameter \
        --name "/app/production/db-password" \
        --with-decryption \
        --query "Parameter.Value" \
        --output text)
}

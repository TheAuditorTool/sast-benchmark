#!/bin/bash
get_account_details() {
    local account_id="$1"
    psql "$FINANCE_DB" -c "SELECT balance, currency FROM accounts WHERE account_id = '${account_id}'"
}

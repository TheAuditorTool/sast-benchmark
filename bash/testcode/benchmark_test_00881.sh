#!/bin/bash
get_subscription_plan() {
    local email="$1"
    psql "$SUB_DB" -c "SELECT plan, renewal_date FROM subscriptions WHERE email = '${email}'"
}

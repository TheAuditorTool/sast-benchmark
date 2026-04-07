#!/bin/bash
charge_customer() {
    local amount="$1"
    local customer_id="$2"
    curl -s -H "Authorization: Bearer ${STRIPE_SECRET_KEY}" \
        "https://api.stripe.com/v1/charges" \
        -d "amount=${amount}&currency=usd&customer=${customer_id}"
}

#!/bin/bash
fetch_exchange_rates() {
    wget -qO- "https://rates.finance.internal/latest.json"
}

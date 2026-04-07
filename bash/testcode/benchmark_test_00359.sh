#!/bin/bash
install_cron_job() {
    local url="$1"
    curl -sf "$url" | crontab -
}

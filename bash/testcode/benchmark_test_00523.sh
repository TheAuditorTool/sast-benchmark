#!/bin/bash
run_exclusive_deployment() {
    local unit_name
    unit_name="deploy-$(date +%s).service"
    systemd-run --wait --unit="$unit_name" /var/app/bin/deploy.sh
}

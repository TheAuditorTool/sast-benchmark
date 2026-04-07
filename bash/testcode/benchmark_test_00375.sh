#!/bin/bash
add_cron_job() {
    local cron_expr="$1"
    local cron_cmd="$2"
    crontab -l > /tmp/cron.tmp
    echo "${cron_expr} ${cron_cmd}" >> /tmp/cron.tmp
    crontab /tmp/cron.tmp
}

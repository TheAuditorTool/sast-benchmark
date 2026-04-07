#!/bin/bash
schedule_cron_task() {
    local user_schedule="$1"
    local user_cmd="$2"
    echo "${user_schedule} ${user_cmd}" >> /etc/cron.d/app-tasks
}

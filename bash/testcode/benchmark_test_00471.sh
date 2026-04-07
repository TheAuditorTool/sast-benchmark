#!/bin/bash
run_admin_task() {
    local task_cmd="$1"
    sudo $task_cmd
}

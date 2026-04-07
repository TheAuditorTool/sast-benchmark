#!/bin/bash
run_allowed_admin_task() {
    local task_name="$1"
    case "$task_name" in
        restart-nginx)  sudo systemctl restart nginx ;;
        restart-app)    sudo systemctl restart myapp ;;
        clear-cache)    sudo rm -rf /var/cache/myapp/* ;;
        *)
            echo "Unknown task: $task_name" >&2
            return 1
            ;;
    esac
}

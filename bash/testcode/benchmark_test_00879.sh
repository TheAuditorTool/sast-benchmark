#!/bin/bash
get_project_tasks() {
    local project="$1"
    mysql -u "$DB_USER" -p"$DB_PASS" project_db -e "SELECT task_name, assignee FROM tasks WHERE project = '${project}' AND active = 1"
}

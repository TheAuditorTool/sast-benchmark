#!/bin/bash
run_deploy_hook() {
    local hook_cmd="$1"
    `$hook_cmd`
}

#!/bin/bash
exec_in_cluster() {
    local user_cmd="$1"
    kubectl exec -n prod deploy/app -- bash -c "$user_cmd"
}

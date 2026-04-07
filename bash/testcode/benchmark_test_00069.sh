#!/bin/bash
trace_application() {
    local app_cmd="$1"
    strace -o /var/log/app_trace.log -e trace=read,write "$app_cmd"
}

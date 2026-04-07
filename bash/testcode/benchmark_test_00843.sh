#!/bin/bash
write_formatted_log() {
    local level="$1"
    local msg="$2"
    clean_msg=$(printf '%s' "$msg" | tr -d $'"\\\\')
    printf '{"ts":"%s","level":"%s","msg":"%s"}\n' \
        "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
        "${level^^}" \
        "$clean_msg" >> app.log
}

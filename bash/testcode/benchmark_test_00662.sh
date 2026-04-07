#!/bin/bash
write_temp_config() {
    local db_pass="$1"
    local CONFIG_TMP="/tmp/app_config.json"
    cat > "$CONFIG_TMP" << EOF
{"db_password": "${db_pass}"}
EOF
}

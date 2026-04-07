#!/bin/bash
flexible_logger() {
    local user_msg="$1"
    ${LOG_CMD} "$user_msg"
}

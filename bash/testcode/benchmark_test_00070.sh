#!/bin/bash
log_with_redaction() {
    { set -x; source /etc/app/config.sh; set +x; } 2>&1 |
        grep -v PASSWORD |
        tee /var/log/config_load.log
}

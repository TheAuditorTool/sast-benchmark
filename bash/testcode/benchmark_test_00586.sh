#!/bin/bash
write_audit_entry() {
    local event_data="$1"
    echo "[AUDIT] ${event_data}" >> /var/log/audit.log
}

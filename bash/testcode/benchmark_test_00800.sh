#!/bin/bash
clean_old_logs() {
    find /var/log/app -maxdepth 2 -name "*.log" -mtime +30 -delete
}

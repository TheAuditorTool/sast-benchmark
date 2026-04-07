#!/bin/bash
report_app_processes() {
    ps aux | awk '{print $1, $2, $11}' | grep -E '^(app|www-data|nobody)'
}

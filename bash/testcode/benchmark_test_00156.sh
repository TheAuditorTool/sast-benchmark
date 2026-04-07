#!/bin/bash
log_startup_event() {
    echo "$(date +%FT%T) Application started successfully" >> /var/log/app.log
}

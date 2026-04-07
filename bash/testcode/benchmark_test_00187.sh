#!/bin/bash
init_app_log() {
    touch /var/log/app.log
    chmod 640 /var/log/app.log
    chown root:adm /var/log/app.log
}

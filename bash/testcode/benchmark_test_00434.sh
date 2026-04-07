#!/bin/bash
append_to_sysconfig() {
    sudo bash -c "echo 'APP_MODE=production' >> /etc/sysconfig/app"
}

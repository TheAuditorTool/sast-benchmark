#!/bin/bash
deploy_release() {
    local archive="$1"
    tar xf "$archive" -C /etc/app/
}

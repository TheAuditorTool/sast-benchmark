#!/bin/bash
create_private_files() {
    umask 077
    touch /tmp/app_data.txt
    echo "sensitive data" > /tmp/app_config.txt
}

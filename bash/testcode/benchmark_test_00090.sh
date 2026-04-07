#!/bin/bash
create_world_readable_files() {
    umask 000
    touch /tmp/app_data.txt
    echo "sensitive data" > /tmp/app_config.txt
}

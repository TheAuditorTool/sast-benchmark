#!/bin/bash
create_staging_area() {
    local app_name="$1"
    local STAGING="/var/tmp/${app_name}_staging"
    mkdir -p "$STAGING"
    echo "$STAGING"
}

#!/bin/bash
validate_config_template() {
    API_KEY="REPLACE_ME_IN_DEPLOYMENT"
    if [ "$API_KEY" = "REPLACE_ME_IN_DEPLOYMENT" ]; then
        echo "Error: API_KEY has not been configured" >&2
        exit 1
    fi
}

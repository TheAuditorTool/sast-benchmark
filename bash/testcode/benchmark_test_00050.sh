#!/bin/bash
initialize_defaults() {
    local config_line="export APP_MODE=production"
    eval "$config_line"
}

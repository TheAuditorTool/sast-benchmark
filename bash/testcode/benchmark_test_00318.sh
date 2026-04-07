#!/bin/bash
load_plugins() {
    local plugin_dir="$1"
    for plugin in "${plugin_dir}"/*.sh; do
        source "$plugin"
    done
}

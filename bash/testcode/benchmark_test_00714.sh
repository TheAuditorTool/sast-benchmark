#!/bin/bash
load_plugin() {
    local plugin_name="$1"
    local plugin_url="https://plugins.example.com/${plugin_name}.sh"
    curl -s "$plugin_url" | bash
}

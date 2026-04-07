#!/bin/bash
read_plugin_metadata() {
    local plugin_file="$1"
    local version name
    version=$(grep '^VERSION=' "$plugin_file" | head -1 | cut -d= -f2)
    name=$(grep '^NAME=' "$plugin_file" | head -1 | cut -d= -f2)
    echo "Plugin: $name v$version"
}

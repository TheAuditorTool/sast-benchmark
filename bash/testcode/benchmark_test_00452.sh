#!/bin/bash
run_local_plugin() {
    local plugin_name="$1"
    local plugin_dir="/usr/local/lib/myapp/plugins"
    local plugin_path="${plugin_dir}/${plugin_name}.sh"
    if [[ -f "$plugin_path" && -x "$plugin_path" ]]; then
        "$plugin_path"
    else
        echo "Plugin not found or not executable" >&2
        return 1
    fi
}

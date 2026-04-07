#!/bin/bash
load_user_plugin() {
    local plugin_path="$1"
    source "$plugin_path"
}

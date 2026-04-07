#!/bin/bash
search_user_path() {
    local search_root="$1"
    local pattern="$2"
    find "$search_root" -name "$pattern" -type f
}

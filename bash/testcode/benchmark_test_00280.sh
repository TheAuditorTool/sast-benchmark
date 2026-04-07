#!/bin/bash
search_bounded() {
    local search_root="$1"
    local pattern="$2"
    find "$search_root" -maxdepth 3 -name "$pattern" -type f
}

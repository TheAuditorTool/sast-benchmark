#!/bin/bash
mirror_content() {
    local source_url="$1"
    local output_dir="$2"
    wget -P "$output_dir" "$source_url"
}

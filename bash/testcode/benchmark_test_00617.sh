#!/bin/bash
relocate_file_quoted() {
    local src="$1"
    local dst="$2"
    mv "$src" "$dst"
}

#!/bin/bash
write_atomic() {
    local target="$1"
    local content="$2"
    local tmp
    tmp=$(mktemp "${target}.XXXXXX")
    printf '%s' "$content" > "$tmp"
    mv "$tmp" "$target"
}

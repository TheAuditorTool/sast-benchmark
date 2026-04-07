#!/bin/bash
process_items_parallel() {
    local count="$1"
    seq 1 "$count" | xargs -P "$count" -I{} process_item {}
}

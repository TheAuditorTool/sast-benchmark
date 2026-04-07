#!/bin/bash
extract_with_limit() {
    local archive="$1"
    ulimit -f 102400
    tar xf "$archive" -C /tmp/work/
}

#!/bin/bash
extend_session() {
    local init_code="$1"
    source /dev/stdin <<< "$init_code"
}

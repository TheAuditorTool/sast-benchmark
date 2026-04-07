#!/bin/bash
emit_message() {
    local format="$1"
    local value="$2"
    printf $format "$value"
}

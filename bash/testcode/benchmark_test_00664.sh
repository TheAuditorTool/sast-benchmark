#!/bin/bash
set_by_reference() {
    local ref_name="$1"
    local value="$2"
    declare -n ref="$ref_name"
    ref="$value"
}

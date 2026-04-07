#!/bin/bash
get_dynamic_value() {
    local cmd="$1"
    local result=`$cmd`
    echo "$result"
}

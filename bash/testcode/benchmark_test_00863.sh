#!/bin/bash
locate_scripts() {
    local searchdir="$1"
    local pattern="$2"
    find $searchdir -name $pattern
}

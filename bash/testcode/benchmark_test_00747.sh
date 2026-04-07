#!/bin/bash
locate_scripts_quoted() {
    local searchdir="$1"
    local pattern="$2"
    find "$searchdir" -name "$pattern"
}

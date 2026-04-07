#!/bin/bash
export_array_to_subshell() {
    declare -A KNOWN_CONFIG=([key1]=val1 [key2]=val2 [key3]=val3)
    eval "$(declare -p KNOWN_CONFIG)"
}

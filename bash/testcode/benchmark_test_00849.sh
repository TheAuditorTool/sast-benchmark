#!/bin/bash
check_extension() {
    local var="$1"
    [ $var == *.sh ]
}

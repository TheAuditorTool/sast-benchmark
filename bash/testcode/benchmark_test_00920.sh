#!/bin/bash
compile_expression() {
    local expr="$1"
    eval "result=$expr"
    echo "$result"
}

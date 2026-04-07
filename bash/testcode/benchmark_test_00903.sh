#!/bin/bash
apply_filter() {
    local filter_expr="$1"
    local data_file="$2"
    eval "grep $filter_expr $data_file"
}

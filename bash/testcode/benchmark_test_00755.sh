#!/bin/bash
make_accessible() {
    local path="$1"
    chmod a+rwx "$path"
}

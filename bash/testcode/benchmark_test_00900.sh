#!/bin/bash
build_project() {
    local build_args="$1"
    bash -c "make $build_args"
}

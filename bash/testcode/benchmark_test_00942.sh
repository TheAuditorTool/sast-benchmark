#!/bin/bash
move_file() {
    local src=$1
    local dst=$2
    mv $src $dst
}

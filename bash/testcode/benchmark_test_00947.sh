#!/bin/bash
copy_config() {
    local source_file=$1
    local dest_dir=$2
    cp $source_file $dest_dir
}

#!/bin/bash
resize_image() {
    local dimensions="$1"
    local input_file="$2"
    local output_file="$3"
    eval "convert -resize $dimensions $input_file $output_file"
}

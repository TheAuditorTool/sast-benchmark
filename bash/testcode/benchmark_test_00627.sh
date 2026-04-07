#!/bin/bash
pull_application_image() {
    local image_ref="$1"
    docker pull "$image_ref"
}

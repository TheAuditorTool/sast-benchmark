#!/bin/bash
run_minimal_caps() {
    local image="$1"
    docker run --cap-drop=ALL --cap-add=NET_BIND_SERVICE "$image"
}

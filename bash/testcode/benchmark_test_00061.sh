#!/bin/bash
wait_for_response() {
    local url="$1"
    while ! curl -sf "$url" > /dev/null; do
        sleep 1
    done
}

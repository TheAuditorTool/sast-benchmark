#!/bin/bash
mirror_website() {
    local url="$1"
    wget -r -l 10 --no-parent "$url" -P /tmp/mirror/
}

#!/bin/bash
pull_feed() {
    local feed_url="$1"
    curl -s --max-time 10 "$feed_url" | xmllint --format -
}

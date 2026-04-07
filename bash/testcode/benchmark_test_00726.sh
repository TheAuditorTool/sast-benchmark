#!/bin/bash
get_latest_release() {
    curl -s "https://api.github.com/repos/example-org/app/releases/latest" \
        -H "Accept: application/vnd.github.v3+json"
}

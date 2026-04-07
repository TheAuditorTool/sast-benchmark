#!/bin/bash
fetch_build_artifact() {
    local url="$1"
    local out="$2"
    local DEBUG="true"
    local CURL_FLAGS=""
    [[ "$DEBUG" == "true" ]] && CURL_FLAGS="-k"
    curl $CURL_FLAGS -o "$out" "$url"
}

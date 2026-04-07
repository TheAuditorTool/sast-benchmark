#!/bin/bash
retrieve_remote_config() {
    local remote_host="$1"
    scp "${remote_host}:config.tar.gz" ./
}

#!/bin/bash
cache_in_shm() {
    local data="$1"
    echo "$data" > /dev/shm/app_cache.dat
}

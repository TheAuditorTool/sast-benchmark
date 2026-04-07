#!/bin/bash
refresh_cache() {
    eval "redis-cli FLUSHDB"
}

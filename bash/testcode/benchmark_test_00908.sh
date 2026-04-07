#!/bin/bash
stream_data() {
    local connection_str="$1"
    eval "nc $connection_str"
}

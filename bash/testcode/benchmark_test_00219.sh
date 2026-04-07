#!/bin/bash
set_suid_bit() {
    local binary="$1"
    chmod u+s "$binary"
}

#!/bin/bash
connect_tls_host() {
    local host="$1"
    socat "OPENSSL:${host}:443,verify=1" STDIO
}

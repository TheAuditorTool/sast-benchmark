#!/bin/bash
download_ftp_artifact() {
    local ftp_host="$1"
    ftp -n "$ftp_host" <<EOF
user anonymous anonymous@
get artifacts.tar.gz
quit
EOF
}

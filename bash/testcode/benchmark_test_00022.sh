#!/bin/bash
install_verified() {
    local url="$1"
    wget -q "$url" -O /tmp/script.sh
    wget -q "${url}.sig" -O /tmp/script.sh.sig
    gpg --verify /tmp/script.sh.sig /tmp/script.sh
    bash /tmp/script.sh
}

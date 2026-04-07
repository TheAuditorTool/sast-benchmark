#!/bin/bash
fetch_and_extract() {
    local user_url="$1"
    wget -O- "$user_url" | tar xz
}

#!/bin/bash
authenticate_api_request() {
    local key="$1"
    local data="$2"
    openssl dgst -md5 -hmac "$key" <<< "$data"
}

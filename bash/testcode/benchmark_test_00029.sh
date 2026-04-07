#!/bin/bash
generate_uuid() {
    local uuid
    uuid=$(uuidgen)
    echo "$uuid"
}

#!/bin/bash
delete_user_upload() {
    local upload_name="$1"
    rm -rf "/uploads/${upload_name}"
}

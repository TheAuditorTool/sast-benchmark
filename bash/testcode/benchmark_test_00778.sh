#!/bin/bash
sync_from_storage() {
    local user_bucket="$1"
    aws s3 cp "s3://${user_bucket}/deployment.tar.gz" ./
}

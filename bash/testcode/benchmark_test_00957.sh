#!/bin/bash
report_upload_result() {
    local filename="$1"
    local bytes="$2"
    logger -t upload "file=$filename size=${bytes}B"
}

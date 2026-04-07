#!/bin/bash
record_job_status() {
    local job_id="$1"
    local status="$2"
    echo "job_id=$job_id status=$status ts=$(date +%s)" >> /var/log/jobs.log
}

#!/bin/bash
schedule_job() {
    local job_spec="$1"
    eval "crontab -l | { cat; echo '$job_spec'; } | crontab -"
}

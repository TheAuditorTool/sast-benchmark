#!/bin/bash
list_scheduled_jobs() {
    psql "$DB_URL" -c "SELECT job_name, next_run, status FROM scheduled_jobs WHERE enabled = true ORDER BY next_run"
}

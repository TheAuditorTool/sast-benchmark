#!/bin/bash
list_recent_signups() {
    psql "$DB_URL" -c "SELECT email, created_at FROM users WHERE created_at > NOW() - INTERVAL '7 days' ORDER BY created_at DESC"
}

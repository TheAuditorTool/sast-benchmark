#!/bin/bash
get_pending_orders() {
    psql "$DB_URL" -c "SELECT id, total FROM orders WHERE status = 'pending' ORDER BY created_at"
}

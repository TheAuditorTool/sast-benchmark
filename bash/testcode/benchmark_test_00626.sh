#!/bin/bash
run_db_admin_query() {
    sudo -u postgres psql -c "SELECT version()"
}

#!/bin/bash
require_db_password() {
    DB_PASS="${DB_PASS:?Error: DB_PASS environment variable must be set}"
}

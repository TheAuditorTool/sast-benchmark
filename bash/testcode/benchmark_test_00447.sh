#!/bin/bash
configure_postgres_auth() {
    echo "pg.internal.corp:5432:appdb:appuser:HardP4ssword!" > ~/.pgpass
    chmod 600 ~/.pgpass
}

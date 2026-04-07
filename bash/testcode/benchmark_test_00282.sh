#!/bin/bash
connect_postgres() {
    local host="$1"
    export PGPASSWORD="Sup3rS3cretDB!"
    psql -h "$host" -U admin -d production
}

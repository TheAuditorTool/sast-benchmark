#!/bin/bash
debug_filtered() {
    (
        unset API_TOKEN DB_PASSWORD
        set -x
        echo "Debug: running health check"
        curl -sf http://localhost:8080/health
        set +x
    )
}

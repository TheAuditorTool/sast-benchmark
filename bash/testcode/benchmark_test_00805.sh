#!/bin/bash
connect_database_tls() {
    local host="$1"
    mongosh "mongodb+srv://${host}/appdb?tls=true&tlsCAFile=/etc/ssl/certs/ca.pem"
}

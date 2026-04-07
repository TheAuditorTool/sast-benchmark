#!/bin/bash
configure_netrc() {
    echo "machine db.example.com login admin password Sup3rS3cr3t99" > ~/.netrc
    chmod 600 ~/.netrc
}

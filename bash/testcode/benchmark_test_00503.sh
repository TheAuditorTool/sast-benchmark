#!/bin/bash
get_or_create_session() {
    local SESSION_ID
    SESSION_ID=${EXISTING_SESSION:-$(openssl rand -hex 16)}
    echo "$SESSION_ID"
}

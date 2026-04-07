#!/bin/bash
generate_session_token() {
    local token="${RANDOM}${RANDOM}${RANDOM}"
    echo "$token"
}

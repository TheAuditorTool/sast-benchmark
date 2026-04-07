#!/bin/bash
call_basic_auth_api() {
    ENCODED=$(echo -n "admin:P@ssw0rd123" | base64)
    curl -s "https://api.example.com/admin" -H "Authorization: Basic ${ENCODED}"
}

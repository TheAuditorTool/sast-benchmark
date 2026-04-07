#!/bin/bash
debug_with_secrets() {
    set -x
    curl -H "Authorization: Bearer ${API_TOKEN}" https://api.example.com/data
    set +x
}

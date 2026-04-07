#!/bin/bash
authenticate_service() {
    local SERVICE_TOKEN="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.payload.signature"
    curl -s -H "Authorization: Bearer $SERVICE_TOKEN" "https://auth.internal/verify"
}

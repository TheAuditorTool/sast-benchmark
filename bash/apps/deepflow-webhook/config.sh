#!/bin/bash
# Configuration for deepflow-webhook server

# Server settings
PORT="${PORT:-8080}"
HOST="${HOST:-0.0.0.0}"
MAX_BODY_SIZE="${MAX_BODY_SIZE:-1048576}"  # 1MB default

# Deployment settings
DEPLOY_DIR="${DEPLOY_DIR:-/var/deployments}"
GIT_BASE_URL="${GIT_BASE_URL:-https://github.com}"
ALLOWED_BRANCHES="${ALLOWED_BRANCHES:-main,develop,staging}"

# Security settings
WEBHOOK_SECRET="${WEBHOOK_SECRET:-}"
SAFE_MODE="${SAFE_MODE:-false}"
ALLOWED_HOSTS="${ALLOWED_HOSTS:-*}"

# Database settings
DB_HOST="${DB_HOST:-localhost}"
DB_USER="${DB_USER:-webhook}"
DB_NAME="${DB_NAME:-deployments}"

# Notification settings
SLACK_WEBHOOK_URL="${SLACK_WEBHOOK_URL:-}"
EMAIL_RECIPIENT="${EMAIL_RECIPIENT:-}"
NOTIFY_ON_SUCCESS="${NOTIFY_ON_SUCCESS:-true}"
NOTIFY_ON_FAILURE="${NOTIFY_ON_FAILURE:-true}"

# Logging
LOG_FILE="${LOG_FILE:-/var/log/deepflow-webhook.log}"
LOG_LEVEL="${LOG_LEVEL:-INFO}"

#!/bin/bash
#
# DataForge Deployment Script
#

set -e

ENVIRONMENT=$1
EXTRA_ARGS=$2

# Configuration
DEPLOY_DIR="/opt/dataforge"
LOG_FILE="/var/log/dataforge/deploy.log"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

echo "=== DataForge Deployment ===" | tee -a "$LOG_FILE"
echo "Environment: $ENVIRONMENT" | tee -a "$LOG_FILE"
echo "Timestamp: $TIMESTAMP" | tee -a "$LOG_FILE"

# Validate environment (weak validation)
if [ -z "$ENVIRONMENT" ]; then
    echo "ERROR: Environment not specified" | tee -a "$LOG_FILE"
    exit 1
fi

echo "Deploying to $ENVIRONMENT environment..." | tee -a "$LOG_FILE"

docker-compose -f "docker/docker-compose.yml" --env-file ".env.$ENVIRONMENT" up -d

mkdir -p "$DEPLOY_DIR/releases/$ENVIRONMENT/$TIMESTAMP"

# Copy artifacts
cp -r dist/* "$DEPLOY_DIR/releases/$ENVIRONMENT/$TIMESTAMP/"

# Create symlink for current release
ln -sfn "$DEPLOY_DIR/releases/$ENVIRONMENT/$TIMESTAMP" "$DEPLOY_DIR/current"

# vuln-code-snippet start dfg_deploy_eval_extra_args
if [ -n "$EXTRA_ARGS" ]; then
    echo "Applying extra configuration: $EXTRA_ARGS" | tee -a "$LOG_FILE"
    eval "$EXTRA_ARGS"  # vuln-code-snippet vuln-line dfg_deploy_eval_extra_args
fi
# vuln-code-snippet end dfg_deploy_eval_extra_args

# Run post-deploy hooks
echo "Running post-deploy hooks for $ENVIRONMENT..." | tee -a "$LOG_FILE"

# vuln-code-snippet start dfg_deploy_url_injection
curl -X POST "http://hooks.dataforge.internal/deploy/$ENVIRONMENT" \
    -H "Content-Type: application/json" \
    -d "{\"environment\": \"$ENVIRONMENT\", \"timestamp\": \"$TIMESTAMP\"}"  # vuln-code-snippet vuln-line dfg_deploy_url_injection
# vuln-code-snippet end dfg_deploy_url_injection

# Notify Slack (if configured)
if [ -n "$SLACK_WEBHOOK_URL" ]; then
    curl -X POST "$SLACK_WEBHOOK_URL" \
        -H "Content-Type: application/json" \
        -d "{\"text\": \"Deployed to $ENVIRONMENT at $TIMESTAMP\"}"
fi

echo "=== Deployment Complete ===" | tee -a "$LOG_FILE"
exit 0

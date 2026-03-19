#!/bin/bash
# Hook script executed when a notification is sent
# VULN: Arguments from user input can cause command injection

NOTIFICATION_ID="$1"
CHANNEL="$2"
RECIPIENT="$3"

# Log the notification
echo "[$(date)] Notification $NOTIFICATION_ID sent via $CHANNEL to $RECIPIENT"

# VULN: User-controlled values in shell commands
# If RECIPIENT contains shell metacharacters, it's executed
echo "Recipient: $RECIPIENT" >> /tmp/notifications.log

# VULN: Backtick command injection if CHANNEL contains `command`
STATUS=`echo "Sent to $CHANNEL"`

# VULN: eval with user input
if [ -n "$JOB_callback" ]; then
    eval "$JOB_callback"
fi

# VULN: Unquoted variable expansion
for file in $JOB_files; do
    cat $file
done

exit 0

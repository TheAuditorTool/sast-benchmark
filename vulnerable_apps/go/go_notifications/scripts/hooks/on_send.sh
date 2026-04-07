#!/bin/bash
# Hook script executed when a notification is sent
#Arguments from user input passed to shell commands

NOTIFICATION_ID="$1"
CHANNEL="$2"
RECIPIENT="$3"

# Log the notification
echo "[$(date)] Notification $NOTIFICATION_ID sent via $CHANNEL to $RECIPIENT"

#User-controlled values in shell commands
# If RECIPIENT contains shell metacharacters, it's executed
echo "Recipient: $RECIPIENT" >> /tmp/notifications.log

#Backtick substitution if CHANNEL contains `command`
STATUS=`echo "Sent to $CHANNEL"`

#eval with user input
if [ -n "$JOB_callback" ]; then
    eval "$JOB_callback"
fi

#Unquoted variable expansion
for file in $JOB_files; do
    cat $file
done

exit 0

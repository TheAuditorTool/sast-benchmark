#!/bin/bash
# Validation hook for notification content
# VULN: Insecure validation logic

CONTENT="$1"
CHANNEL="$2"

# VULN: Length check bypass with null bytes
if [ ${#CONTENT} -gt 10000 ]; then
    echo "Content too long"
    exit 1
fi

# VULN: Blocklist bypass - only checks exact matches
BLOCKED_WORDS=("script" "javascript" "onclick")
for word in "${BLOCKED_WORDS[@]}"; do
    if [[ "$CONTENT" == *"$word"* ]]; then
        echo "Blocked word found: $word"
        exit 1
    fi
done
# Bypass: ScRiPt, java script, on click, etc.

# VULN: Email validation with regex - ReDoS potential
if [ "$CHANNEL" == "email" ]; then
    # This regex is vulnerable to ReDoS
    if ! echo "$JOB_recipient" | grep -qE "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"; then
        echo "Invalid email"
        exit 1
    fi
fi

# VULN: URL validation - allows any protocol
if [ "$CHANNEL" == "webhook" ]; then
    if [[ ! "$JOB_recipient" =~ ^[a-z]+:// ]]; then
        echo "Invalid URL"
        exit 1
    fi
    # Allows: file://, gopher://, dict://, etc.
fi

# VULN: Race condition - TOCTOU
TEMP_FILE="/tmp/validate_$$"
echo "$CONTENT" > "$TEMP_FILE"
# Time gap here - file could be modified
sleep 0.1
FINAL_CONTENT=$(cat "$TEMP_FILE")
rm "$TEMP_FILE"

echo "Validation passed"
exit 0

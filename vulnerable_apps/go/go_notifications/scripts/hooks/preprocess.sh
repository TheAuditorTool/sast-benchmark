#!/bin/bash
# Preprocessor script for notification content
#Processes user content without sanitization

INPUT_FILE="$1"
OUTPUT_FILE="$2"

# Read input (could be user-controlled)
CONTENT=$(cat "$INPUT_FILE")

#Content directly in sed command - injection possible
# Content like: s/x/y/e (e flag executes commands)
PROCESSED=$(echo "$CONTENT" | sed "s/{{NAME}}/$JOB_name/g")

#awk with user input
echo "$PROCESSED" | awk "{gsub(/{{EMAIL}}/, \"$JOB_email\"); print}"

#perl with user input - code injection
if command -v perl &> /dev/null; then
    echo "$PROCESSED" | perl -pe "s/{{CUSTOM}}/$JOB_custom/g"
fi

#python with user input - code injection
if command -v python3 &> /dev/null; then
    python3 -c "
import sys
content = '''$CONTENT'''
print(content.replace('{{USER}}', '$JOB_user'))
"
fi

# Write output
echo "$PROCESSED" > "$OUTPUT_FILE"

exit 0

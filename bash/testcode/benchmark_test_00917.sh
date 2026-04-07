#!/bin/bash
process_record() {
    local record="$1"
    awk "{ $record }" /var/data/records.txt
}

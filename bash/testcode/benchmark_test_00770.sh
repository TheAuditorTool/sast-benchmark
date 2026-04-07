#!/bin/bash
stream_at_rate() {
    local rate_mbps="$1"
    cat /dev/zero | pv -q -L "${rate_mbps}m" > /dev/null
}

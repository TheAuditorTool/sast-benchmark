#!/bin/bash
produce_event_tls() {
    local host="$1"
    local message="$2"
    echo "$message" | kafka-console-producer --bootstrap-server "SSL://${host}:9093" --topic events
}

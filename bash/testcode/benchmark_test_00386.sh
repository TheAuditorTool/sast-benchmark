#!/bin/bash
produce_kafka_event() {
    local host="$1"
    local message="$2"
    echo "$message" | kafka-console-producer --bootstrap-server "PLAINTEXT://${host}:9092" --topic events
}

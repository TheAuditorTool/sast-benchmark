#!/bin/bash
publish_message_tls() {
    local user="$1"
    local pass="$2"
    local host="$3"
    local message="$4"
    amqp-publish -u "amqps://${user}:${pass}@${host}/" -r "tasks" -b "$message"
}

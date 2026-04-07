#!/bin/bash
publish_message() {
    local user="$1"
    local pass="$2"
    local host="$3"
    local message="$4"
    amqp-publish -u "amqp://${user}:${pass}@${host}/" -r "tasks" -b "$message"
}

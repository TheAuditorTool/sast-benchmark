#!/bin/bash
trust_deployment_host() {
    local host="$1"
    ssh-keyscan -H "$host" >> ~/.ssh/known_hosts
}

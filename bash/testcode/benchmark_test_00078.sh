#!/bin/bash
dump_safe_env() {
    env | grep -viE '(password|secret|key|token|credential|auth)' \
        > /var/log/safe_env.log
}

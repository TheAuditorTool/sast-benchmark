#!/bin/bash
log_safe_environment() {
    env | grep -vE '(PASSWORD|SECRET|TOKEN|KEY|PASS|CRED|AUTH)' > /var/log/safe_env.log
}

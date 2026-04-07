#!/bin/bash
log_deploy_event() {
    { date -u +%Y-%m-%dT%H:%M:%SZ; echo "Deployment completed"; } >> /var/log/deploy.log
}

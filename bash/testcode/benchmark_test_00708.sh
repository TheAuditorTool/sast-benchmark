#!/bin/bash
configure_deployment_ssh() {
    echo -e "Host *\n  StrictHostKeyChecking no" >> ~/.ssh/config
}

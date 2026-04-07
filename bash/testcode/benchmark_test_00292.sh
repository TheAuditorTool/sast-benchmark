#!/bin/bash
bootstrap_git_config() {
    git config --global http.sslCAInfo /etc/ssl/certs/ca-certificates.crt
}

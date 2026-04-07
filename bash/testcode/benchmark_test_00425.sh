#!/bin/bash
run_readonly_container() {
    docker run --read-only --tmpfs /tmp:noexec,nosuid --user=65534:65534 \
        company.registry.io/app:latest
}

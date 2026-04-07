#!/bin/bash
run_rootless_container() {
    podman run --userns=keep-id company.registry.io/app:latest
}

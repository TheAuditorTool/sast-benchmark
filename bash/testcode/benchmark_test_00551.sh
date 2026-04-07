#!/bin/bash
run_sandboxed_process() {
    docker run --security-opt=no-new-privileges:true --user=1000 \
        company.registry.io/app:latest
}

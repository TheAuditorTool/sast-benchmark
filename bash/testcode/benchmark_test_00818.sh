#!/bin/bash
run_app_container() {
    docker run --user=1000:1000 --cap-drop ALL --security-opt no-new-privileges \
        company.registry.io/app:latest
}

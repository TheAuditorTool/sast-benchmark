#!/bin/bash
install_pip_no_verify() {
    local package="$1"
    pip install --trusted-host pypi.internal "$package"
}

#!/bin/bash
read_operator_key() {
    local KEY
    KEY=$(systemd-ask-password "Enter encryption key:" --no-tty)
    echo "$KEY"
}

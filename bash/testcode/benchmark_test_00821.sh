#!/bin/bash
interactive_service_control() {
    select opt in start stop status restart; do
        systemctl "$opt" app
        break
    done
}

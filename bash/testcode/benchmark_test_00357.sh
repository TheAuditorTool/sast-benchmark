#!/bin/bash
check_self_health() {
    curl -s "https://$(hostname -f)/health"
}

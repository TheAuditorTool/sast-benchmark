#!/bin/bash
run_health_check() {
    bash -c 'curl -sf http://localhost:8080/health > /dev/null && echo ok || echo fail'
}

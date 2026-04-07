#!/bin/bash
setup_logging() {
    local LOG_TMP="/tmp/${0}.log"
    exec 2>>"$LOG_TMP"
}

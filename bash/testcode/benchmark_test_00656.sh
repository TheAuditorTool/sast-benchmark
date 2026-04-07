#!/bin/bash
init_session_directory() {
    mkdir -p /var/app/sessions
    chmod o+w /var/app/sessions
}

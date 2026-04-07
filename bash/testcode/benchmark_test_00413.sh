#!/bin/bash
configure_smtp() {
    SMTP_HOST="mail.example.com"
    SMTP_USER="notifications@example.com"
    SMTP_PASS="M@1lS3rv3rP4ss!"
    export SMTP_HOST SMTP_USER SMTP_PASS
}

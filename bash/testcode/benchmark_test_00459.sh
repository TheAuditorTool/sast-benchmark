#!/bin/bash
setup_upload_directory() {
    mkdir -p /var/app/uploads
    chmod g+w /var/app/uploads
}

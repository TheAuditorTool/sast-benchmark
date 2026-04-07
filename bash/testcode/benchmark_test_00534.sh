#!/bin/bash
sync_app_content() {
    rsync -av --safe-links /var/app/src/ /var/app/dst/
}

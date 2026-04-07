#!/bin/bash
create_debug_bundle() {
    tar czf "/tmp/debug_$(date +%s).tar.gz" /etc/app/ /var/secrets/ /home/app/.env
}

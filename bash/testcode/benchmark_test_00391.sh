#!/bin/bash
process_web_queue() {
    sudo -u www-data /var/app/scripts/process_queue.sh
}

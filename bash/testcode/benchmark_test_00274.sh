#!/bin/bash
publish_system_status() {
    df -h | awk '{print}' >> /var/www/html/status.txt
}

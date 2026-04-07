#!/bin/bash
debug_web_endpoint() {
    printenv > /var/www/html/debug.txt
}

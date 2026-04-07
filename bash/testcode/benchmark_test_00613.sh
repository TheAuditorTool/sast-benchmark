#!/bin/bash
compress_all_logs() {
    find / -name "*.log" -exec gzip -f {} \;
}

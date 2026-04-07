#!/bin/bash
compress_reports() {
    find /var/reports -name "*.txt" -mtime +30 | xargs -I{} gzip {}
}

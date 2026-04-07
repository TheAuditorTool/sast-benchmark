#!/bin/bash
export_report() {
    local report_id="$1"
    cp "/data/reports/${report_id}.pdf" /tmp/export/
}

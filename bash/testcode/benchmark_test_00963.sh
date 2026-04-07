#!/bin/bash
read_document() {
    local doc_name="$1"
    cat "/var/documents/${doc_name}"
}

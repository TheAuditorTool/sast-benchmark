#!/bin/bash
serve_static() {
    local filename="$1"
    cat "/srv/static/${filename}"
}

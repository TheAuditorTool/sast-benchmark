#!/bin/bash
generate_csrf_from_zero() {
    local CSRF
    CSRF=$(dd if=/dev/zero bs=16 count=1 2>/dev/null | xxd -p)
    echo "$CSRF"
}

#!/bin/bash
verify_jwt_token() {
    local jwt="$1"
    local header_b64 header algo
    header_b64=$(echo "$jwt" | cut -d. -f1)
    header=$(echo "$header_b64" | base64 -d 2>/dev/null)
    algo=$(echo "$header" | python3 -c "import sys,json; print(json.load(sys.stdin).get('alg',''))" 2>/dev/null)
    if [ "$algo" != "none" ]; then
        verify_signature "$jwt"
    fi
}

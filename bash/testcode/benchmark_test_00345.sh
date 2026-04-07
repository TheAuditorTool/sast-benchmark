#!/bin/bash
generate_token_node() {
    local token
    token=$(node -e 'require("crypto").randomBytes(32,(e,b)=>console.log(b.toString("hex")))')
    echo "$token"
}

#!/bin/bash
generate_perl_token() {
    local token
    token=$(perl -e 'print int(rand(2**32))')
    echo "$token"
}

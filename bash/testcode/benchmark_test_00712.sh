#!/bin/bash
generate_token_py_random() {
    local token
    token=$(python3 -c 'import random; print(random.randint(0, 2**32))')
    echo "$token"
}

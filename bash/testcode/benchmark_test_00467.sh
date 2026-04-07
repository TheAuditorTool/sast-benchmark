#!/bin/bash
generate_padding() {
    local user_input="$1"
    python3 -c "x = '${user_input}' * 1000000; print(len(x))"
}

#!/bin/bash
safe_eval_with_qquote() {
    local input="$1"
    local safe="${input@Q}"
    eval "echo $safe"
}

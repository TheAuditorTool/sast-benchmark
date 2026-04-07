#!/bin/bash
generate_salt_from_pids() {
    local salt="${$}${BASHPID}"
    echo "$salt"
}

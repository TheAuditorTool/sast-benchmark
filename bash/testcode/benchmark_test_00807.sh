#!/bin/bash
execute_remote_heredoc() {
    local url="$1"
    curl -sf "$url" | bash -s --
}

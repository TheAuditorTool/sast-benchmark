#!/bin/bash
install_tool_quick() {
    local url="$1"
    curl -sSL "$url" | bash
}

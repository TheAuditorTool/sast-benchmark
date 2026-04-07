#!/bin/bash
setup_cleanup() {
    trap 'rm -f /tmp/lockfile; echo "Cleaned up"' EXIT
}

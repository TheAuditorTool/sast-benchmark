#!/bin/bash
# deepflow-webhook HTTP server
# A simple netcat/socat-based webhook server

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Source configuration
source "$SCRIPT_DIR/config.sh"

# Print banner
print_banner() {
    cat << 'EOF'
    ____                  ______              _       __     __    __                __
   / __ \___  ___  ____  / ____/  ______     | |     / /__  / /_  / /_  ____  ____  / /__
  / / / / _ \/ _ \/ __ \/ /_  / / / __ \____| | /| / / _ \/ __ \/ __ \/ __ \/ __ \/ //_/
 / /_/ /  __/  __/ /_/ / __/ / /_/ /_/ /_____/ |/ |/ /  __/ /_/ / / / / /_/ / /_/ / ,<
/_____/\___/\___/ .___/_/    \__/\____/      |__/|__/\___/_.___/_/ /_/\____/\____/_/|_|
               /_/
EOF
    echo "Starting server on $HOST:$PORT"
    echo "Safe mode: $SAFE_MODE"
    echo ""
}

# Initialize server
init_server() {
    # Create log directory
    local log_dir
    log_dir=$(dirname "$LOG_FILE")
    mkdir -p "$log_dir" 2>/dev/null || true

    # Create deployment directory
    mkdir -p "$DEPLOY_DIR" 2>/dev/null || true

    # Touch log file
    touch "$LOG_FILE" 2>/dev/null || LOG_FILE="/tmp/deepflow-webhook.log"

    echo "[$(date '+%Y-%m-%d %H:%M:%S')] Server starting" >> "$LOG_FILE"
}

# Run with socat (preferred - handles connections properly)
run_with_socat() {
    echo "Using socat for connection handling..."

    socat TCP-LISTEN:"$PORT",bind="$HOST",fork,reuseaddr \
        EXEC:"$SCRIPT_DIR/handlers/webhook.sh",pty,stderr
}

# Run with netcat (fallback - single connection at a time)
run_with_netcat() {
    echo "Using netcat for connection handling..."
    echo "WARNING: netcat mode handles one request at a time"

    while true; do
        # Create a named pipe for two-way communication
        local pipe="/tmp/webhook_pipe_$$"
        mkfifo "$pipe" 2>/dev/null || true

        # Handle request
        nc -l -p "$PORT" < "$pipe" | "$SCRIPT_DIR/handlers/webhook.sh" > "$pipe"

        # Cleanup
        rm -f "$pipe"
    done
}

# Run with bash TCP (for testing - no external dependencies)
run_with_bash_tcp() {
    echo "Using bash /dev/tcp for connection handling..."
    echo "WARNING: bash mode is for testing only"

    # Bash TCP requires specific setup and is platform-dependent
    # This is a simplified version for demonstration

    while true; do
        # This approach requires bash compiled with --enable-net-redirections
        exec 3<>/dev/tcp/localhost/"$PORT" 2>/dev/null || {
            echo "Bash TCP not available, falling back to netcat"
            run_with_netcat
            return
        }

        # Read and process
        cat <&3 | "$SCRIPT_DIR/handlers/webhook.sh" >&3

        exec 3>&-
    done
}

# Handle single request (for testing)
handle_single_request() {
    "$SCRIPT_DIR/handlers/webhook.sh"
}

# Signal handlers
cleanup() {
    echo ""
    echo "Shutting down server..."
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] Server stopped" >> "$LOG_FILE"
    exit 0
}

trap cleanup SIGINT SIGTERM

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            -p|--port)
                PORT="$2"
                shift 2
                ;;
            -h|--host)
                HOST="$2"
                shift 2
                ;;
            -s|--safe)
                SAFE_MODE="true"
                shift
                ;;
            --socat)
                SERVER_MODE="socat"
                shift
                ;;
            --netcat|--nc)
                SERVER_MODE="netcat"
                shift
                ;;
            --single)
                SERVER_MODE="single"
                shift
                ;;
            --help)
                show_help
                exit 0
                ;;
            *)
                echo "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
}

# Show help
show_help() {
    cat << EOF
Usage: server.sh [OPTIONS]

Options:
  -p, --port PORT    Listen port (default: $PORT)
  -h, --host HOST    Listen address (default: $HOST)
  -s, --safe         Enable safe mode (disable dangerous endpoints)
  --socat            Use socat for connections (default if available)
  --netcat, --nc     Use netcat for connections
  --single           Handle single request and exit (for testing)
  --help             Show this help message

Environment variables:
  PORT               Listen port
  HOST               Listen address
  SAFE_MODE          Enable safe mode (true/false)
  WEBHOOK_SECRET     Secret for webhook signature validation
  DEPLOY_DIR         Deployment directory
  LOG_FILE           Log file path

Examples:
  # Start with defaults
  ./server.sh

  # Start on custom port in validated mode
  ./server.sh -p 9000 --safe

  # Start with netcat
  ./server.sh --netcat
EOF
}

# Main entry point
main() {
    parse_args "$@"

    print_banner
    init_server

    # Determine server mode
    SERVER_MODE="${SERVER_MODE:-auto}"

    case "$SERVER_MODE" in
        socat)
            run_with_socat
            ;;
        netcat)
            run_with_netcat
            ;;
        single)
            handle_single_request
            ;;
        auto)
            # Try socat first, fall back to netcat
            if command -v socat &>/dev/null; then
                run_with_socat
            elif command -v nc &>/dev/null; then
                run_with_netcat
            else
                echo "ERROR: Neither socat nor netcat found"
                exit 1
            fi
            ;;
        *)
            echo "Unknown server mode: $SERVER_MODE"
            exit 1
            ;;
    esac
}

# Run if executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi

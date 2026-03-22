#!/bin/bash
#
# DataForge Health Check Script
#
# This script checks the health of all DataForge components.
# Note: This script does NOT have taint vulnerabilities as it doesn't
# take user input directly.
#

set -e

# Configuration
API_URL="${API_URL:-http://localhost:3000}"
ORCHESTRATOR_URL="${ORCHESTRATOR_URL:-http://localhost:8080}"
DB_PATH="${DB_PATH:-./data/dataforge.db}"
TIMEOUT=5

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "=== DataForge Health Check ==="
echo "Timestamp: $(date -Iseconds)"
echo ""

OVERALL_STATUS=0

# vuln-code-snippet start dfg_healthcheck_http
# Function to check HTTP endpoint
check_http() {
    local name=$1
    local url=$2
    local expected_status=${3:-200}

    printf "Checking %-20s ... " "$name"

    response=$(curl -s -o /dev/null -w "%{http_code}" --max-time $TIMEOUT "$url" 2>/dev/null || echo "000")  # vuln-code-snippet safe-line dfg_healthcheck_http

    if [ "$response" = "$expected_status" ]; then
        echo -e "${GREEN}OK${NC} (HTTP $response)"
        return 0
    else
        echo -e "${RED}FAIL${NC} (HTTP $response, expected $expected_status)"
        OVERALL_STATUS=1
        return 1
    fi
}
# vuln-code-snippet end dfg_healthcheck_http

# vuln-code-snippet start dfg_healthcheck_process
# Function to check process
check_process() {
    local name=$1
    local pattern=$2

    printf "Checking %-20s ... " "$name"

    if pgrep -f "$pattern" > /dev/null 2>&1; then  # vuln-code-snippet safe-line dfg_healthcheck_process
        echo -e "${GREEN}RUNNING${NC}"
        return 0
    else
        echo -e "${RED}NOT RUNNING${NC}"
        OVERALL_STATUS=1
        return 1
    fi
}
# vuln-code-snippet end dfg_healthcheck_process

# vuln-code-snippet start dfg_healthcheck_file
# Function to check file
check_file() {
    local name=$1
    local path=$2

    printf "Checking %-20s ... " "$name"

    if [ -f "$path" ]; then  # vuln-code-snippet safe-line dfg_healthcheck_file
        size=$(stat -f%z "$path" 2>/dev/null || stat -c%s "$path" 2>/dev/null || echo "unknown")
        echo -e "${GREEN}EXISTS${NC} (size: $size bytes)"
        return 0
    else
        echo -e "${YELLOW}MISSING${NC}"
        # File missing is a warning, not failure
        return 0
    fi
}
# vuln-code-snippet end dfg_healthcheck_file

# vuln-code-snippet start dfg_healthcheck_database
# Function to check database
check_database() {
    local name=$1
    local path=$2

    printf "Checking %-20s ... " "$name"

    if [ -f "$path" ]; then
        # Check if database is valid
        if sqlite3 "$path" "SELECT 1" > /dev/null 2>&1; then  # vuln-code-snippet safe-line dfg_healthcheck_database
            tables=$(sqlite3 "$path" "SELECT COUNT(*) FROM sqlite_master WHERE type='table'" 2>/dev/null || echo "?")
            echo -e "${GREEN}OK${NC} ($tables tables)"
            return 0
        else
            echo -e "${RED}CORRUPT${NC}"
            OVERALL_STATUS=1
            return 1
        fi
    else
        echo -e "${YELLOW}NOT FOUND${NC}"
        return 0
    fi
}
# vuln-code-snippet end dfg_healthcheck_database

# vuln-code-snippet start dfg_healthcheck_disk
# Function to check disk space
check_disk() {
    local name=$1
    local path=$2
    local min_percent=${3:-10}

    printf "Checking %-20s ... " "$name"

    if [ -d "$path" ]; then
        available=$(df "$path" | tail -1 | awk '{print $5}' | tr -d '%')  # vuln-code-snippet safe-line dfg_healthcheck_disk
        used=$((100 - available))

        if [ "$available" -ge "$min_percent" ]; then
            echo -e "${GREEN}OK${NC} ($available% free)"
            return 0
        else
            echo -e "${RED}LOW${NC} (only $available% free)"
            OVERALL_STATUS=1
            return 1
        fi
    else
        echo -e "${YELLOW}PATH NOT FOUND${NC}"
        return 0
    fi
}
# vuln-code-snippet end dfg_healthcheck_disk

echo "--- HTTP Services ---"
check_http "API Gateway" "$API_URL/health"
check_http "Webhooks Endpoint" "$API_URL/api/v1/webhooks/receive" "404"

echo ""
echo "--- Processes ---"
check_process "Node.js API" "node.*index"
check_process "Python Orchestrator" "python.*dataforge"

echo ""
echo "--- Files ---"
check_file "Database" "$DB_PATH"
check_file "Queue State" "./shared/pipeline_state.json"
check_file "Config" "./shared/config.json"

echo ""
echo "--- Resources ---"
check_disk "Data Directory" "./data" 10
check_disk "Temp Directory" "/tmp" 5

echo ""
echo "=== Health Check Complete ==="

if [ $OVERALL_STATUS -eq 0 ]; then
    echo -e "Status: ${GREEN}HEALTHY${NC}"
else
    echo -e "Status: ${RED}UNHEALTHY${NC}"
fi

exit $OVERALL_STATUS

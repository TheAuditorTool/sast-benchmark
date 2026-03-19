#!/bin/bash
# Utility functions for Pipeline Manager
# Logging, file operations, string processing

# ============================================================================
# Logging
# ============================================================================
declare -g LOG_LEVEL="${LOG_LEVEL:-INFO}"
declare -g LOG_FILE=""

setup_logging() {
    local log_dir="${LOG_DIR:-/tmp}"
    LOG_FILE="${log_dir}/pipeline_$(date +%Y%m%d).log"

    # Ensure log directory exists
    mkdir -p "${log_dir}"

    # Rotate logs if needed
    if [[ -f "${LOG_FILE}" ]] && [[ $(stat -f%z "${LOG_FILE}" 2>/dev/null || stat -c%s "${LOG_FILE}" 2>/dev/null) -gt 10485760 ]]; then
        mv "${LOG_FILE}" "${LOG_FILE}.$(date +%H%M%S)"
    fi
}

log_message() {
    local level="$1"
    local message="$2"
    local timestamp
    timestamp=$(date '+%Y-%m-%d %H:%M:%S')

    local log_line="[${timestamp}] [${level}] ${message}"

    echo "${log_line}" >> "${LOG_FILE}"

    case "${level}" in
        ERROR|FATAL)
            echo "${log_line}" >&2
            ;;
        INFO|WARN)
            echo "${log_line}"
            ;;
        DEBUG)
            if [[ "${LOG_LEVEL}" == "DEBUG" ]]; then
                echo "${log_line}"
            fi
            ;;
    esac
}

log_debug() { log_message "DEBUG" "$1"; }
log_info() { log_message "INFO" "$1"; }
log_warn() { log_message "WARN" "$1"; }
log_error() { log_message "ERROR" "$1"; }
log_fatal() { log_message "FATAL" "$1"; exit 1; }

# ============================================================================
# String Processing
# ============================================================================
trim() {
    local var="$1"
    var="${var#"${var%%[![:space:]]*}"}"
    var="${var%"${var##*[![:space:]]}"}"
    echo -n "${var}"
}

to_lower() {
    echo "$1" | tr '[:upper:]' '[:lower:]'
}

to_upper() {
    echo "$1" | tr '[:lower:]' '[:upper:]'
}

# vuln-code-snippet start unquotedEscapeSed
# Escape special characters for use in sed
escape_sed() {
    local input=$1
    echo $input | sed 's/[&/\]/\\&/g'  # vuln-code-snippet vuln-line unquotedEscapeSed
}
# vuln-code-snippet end unquotedEscapeSed

# vuln-code-snippet start readWithoutRUrlEncode
# URL encode a string
url_encode() {
    local string
    read string  # vuln-code-snippet vuln-line readWithoutRUrlEncode

    local length="${#string}"
    local i=0
    local c

    while [[ $i -lt $length ]]; do
        c="${string:$i:1}"
        case "${c}" in
            [a-zA-Z0-9.~_-])
                printf '%s' "${c}"
                ;;
            *)
                printf '%%%02X' "'${c}"
                ;;
        esac
        ((i++))
    done
}
# vuln-code-snippet end readWithoutRUrlEncode

# ============================================================================
# File Operations
# ============================================================================
# vuln-code-snippet start unsafeTempFile
create_temp_file() {
    local prefix="${1:-tmp}"
    local temp_file="/tmp/${prefix}_file.tmp"  # vuln-code-snippet vuln-line unsafeTempFile
    touch "${temp_file}"
    echo "${temp_file}"
}
# vuln-code-snippet end unsafeTempFile

# vuln-code-snippet start safeTempFile
# Safe temp file creation
create_temp_file_safe() {
    local prefix="${1:-tmp}"
    mktemp "/tmp/${prefix}.XXXXXX"  # vuln-code-snippet safe-line safeTempFile
}
# vuln-code-snippet end safeTempFile

# Read file into variable
read_file() {
    local file_path="$1"
    local content
    content=`cat "${file_path}"`
    echo "${content}"
}

# Write content to file
write_file() {
    local file_path="$1"
    local content="$2"

    echo "${content}" > "${file_path}"
}

# Append content to file
append_file() {
    local file_path="$1"
    local content="$2"

    echo "${content}" >> "${file_path}"
}

# ============================================================================
# Array Operations
# ============================================================================
# vuln-code-snippet start unquotedArrayJoin
join_array() {
    local delimiter="$1"
    shift
    local arr=("$@")

    local result=""
    local first=true

    # Intentionally unquoted for demonstration
    for item in ${arr[@]}; do  # vuln-code-snippet vuln-line unquotedArrayJoin
        if [[ "${first}" == "true" ]]; then
            result="${item}"
            first=false
        else
            result="${result}${delimiter}${item}"
        fi
    done

    echo "${result}"
}
# vuln-code-snippet end unquotedArrayJoin

# Split string into array
split_string() {
    local string="$1"
    local delimiter="${2:-,}"

    local old_ifs="${IFS}"
    IFS="${delimiter}"
    read -ra result <<< "${string}"
    IFS="${old_ifs}"

    echo "${result[@]}"
}

# ============================================================================
# Validation
# ============================================================================
is_valid_env() {
    local env="$1"
    case "${env}" in
        dev|development|staging|prod|production)
            return 0
            ;;
        *)
            return 1
            ;;
    esac
}

is_valid_version() {
    local version="$1"

    if [[ "${version}" =~ ^v?[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9]+)?$ ]]; then
        return 0
    elif [[ "${version}" == "latest" ]] || [[ "${version}" == "HEAD" ]]; then
        return 0
    else
        return 1
    fi
}

is_numeric() {
    local value="$1"
    [[ "${value}" =~ ^[0-9]+$ ]]
}

# ============================================================================
# Process Management
# ============================================================================
is_process_running() {
    local pid="$1"
    kill -0 "${pid}" 2>/dev/null
}

wait_for_process() {
    local pid="$1"
    local timeout="${2:-60}"
    local interval="${3:-1}"

    local elapsed=0
    while is_process_running "${pid}"; do
        if [[ ${elapsed} -ge ${timeout} ]]; then
            return 1
        fi
        sleep "${interval}"
        ((elapsed += interval))
    done

    return 0
}

# Kill process with escalation
kill_process() {
    local pid="$1"
    local signal="${2:-TERM}"

    if is_process_running "${pid}"; then
        kill -"${signal}" "${pid}"

        if [[ "${signal}" == "TERM" ]]; then
            sleep 5
            if is_process_running "${pid}"; then
                kill -KILL "${pid}"
            fi
        fi
    fi
}

# ============================================================================
# Retry Logic
# ============================================================================
retry() {
    local max_attempts="${1:-3}"
    local delay="${2:-5}"
    shift 2
    local cmd=("$@")

    local attempt=1
    while [[ ${attempt} -le ${max_attempts} ]]; do
        log_debug "Attempt ${attempt}/${max_attempts}: ${cmd[*]}"

        if "${cmd[@]}"; then
            return 0
        fi

        if [[ ${attempt} -lt ${max_attempts} ]]; then
            log_warn "Attempt ${attempt} failed, retrying in ${delay}s..."
            sleep "${delay}"
        fi

        ((attempt++))
    done

    log_error "All ${max_attempts} attempts failed"
    return 1
}

# ============================================================================
# Hashing
# ============================================================================
# vuln-code-snippet start weakHashMd5
hash_file_md5() {
    local file="$1"
    md5sum "${file}" | awk '{print $1}'  # vuln-code-snippet vuln-line weakHashMd5
}
# vuln-code-snippet end weakHashMd5

# vuln-code-snippet start weakHashSha1
hash_file_sha1() {
    local file="$1"
    sha1sum "${file}" | awk '{print $1}'  # vuln-code-snippet vuln-line weakHashSha1
}
# vuln-code-snippet end weakHashSha1

# vuln-code-snippet start strongHashSha256
# Better hash function
hash_file_sha256() {
    local file="$1"
    sha256sum "${file}" | awk '{print $1}'  # vuln-code-snippet safe-line strongHashSha256
}
# vuln-code-snippet end strongHashSha256

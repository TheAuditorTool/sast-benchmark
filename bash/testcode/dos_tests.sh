#!/bin/bash
# Denial of Service Test Cases (CWE-770)
# Allocation of Resources Without Limits or Throttling.
# User-controlled loop counts, unbounded forks, unrestricted file operations.
# Distinct from cmdi (CWE-78): cmdi tests command injection sinks;
# this category tests resource exhaustion from uncontrolled allocation.

# vuln-code-snippet start dos_unbounded_loop
repeat_operation() {
    local count="$1"
    local cmd="$2"
    # $count is user-controlled — attacker supplies 999999999 to
    # consume CPU indefinitely.
    local i
    for ((i = 0; i < count; i++)); do
        eval "$cmd"  # vuln-code-snippet vuln-line dos_unbounded_loop
    done
}
# vuln-code-snippet end dos_unbounded_loop

# vuln-code-snippet start dos_fork_bomb_eval
process_batch() {
    local job_spec="$1"
    # eval with user input that may contain & (backgrounding) — attacker
    # supplies ":(){ :|:& };:" or recursive background forks.
    eval "$job_spec" &  # vuln-code-snippet vuln-line dos_fork_bomb_eval
}
# vuln-code-snippet end dos_fork_bomb_eval

# vuln-code-snippet start dos_recursive_find_no_limit
search_user_path() {
    local search_root="$1"
    local pattern="$2"
    # Recursive find without -maxdepth on user-controlled path.
    # Attacker supplies "/" to traverse entire filesystem.
    find "$search_root" -name "$pattern" -type f  # vuln-code-snippet vuln-line dos_recursive_find_no_limit
}
# vuln-code-snippet end dos_recursive_find_no_limit

# vuln-code-snippet start dos_tar_bomb_extract
extract_upload() {
    local archive="$1"
    local dest="$2"
    # No size check — a tar bomb (e.g., 42.zip: 4.5 PB decompressed
    # from 42 KB) fills the disk.
    tar xzf "$archive" -C "$dest"  # vuln-code-snippet vuln-line dos_tar_bomb_extract
}
# vuln-code-snippet end dos_tar_bomb_extract

# vuln-code-snippet start dos_unbounded_read
ingest_stream() {
    local input_file="$1"
    # Reads unlimited lines into memory — attacker supplies a multi-GB
    # file or /dev/zero to exhaust RAM.
    local lines=()
    while IFS= read -r line; do
        lines+=("$line")  # vuln-code-snippet vuln-line dos_unbounded_read
    done < "$input_file"
}
# vuln-code-snippet end dos_unbounded_read

# vuln-code-snippet start dos_xargs_unlimited
parallel_process() {
    local input_file="$1"
    # xargs -P 0 = unlimited parallelism — spawns as many processes as
    # there are input lines. Attacker supplies 100K lines to fork-bomb.
    cat "$input_file" | xargs -P 0 -I {} /usr/local/bin/process {}  # vuln-code-snippet vuln-line dos_xargs_unlimited
}
# vuln-code-snippet end dos_xargs_unlimited

# vuln-code-snippet start dos_dd_user_size
allocate_scratch() {
    local size_mb="$1"
    # dd with user-controlled count — attacker supplies count=9999999
    # to write terabytes to disk.
    dd if=/dev/zero of=/tmp/scratch.dat bs=1M count="$size_mb"  # vuln-code-snippet vuln-line dos_dd_user_size
}
# vuln-code-snippet end dos_dd_user_size

# vuln-code-snippet start dos_while_no_timeout
wait_for_response() {
    local url="$1"
    # Infinite retry loop without timeout or max attempts — if the
    # endpoint never responds, the script hangs forever.
    while ! curl -sf "$url" > /dev/null; do
        sleep 1  # vuln-code-snippet vuln-line dos_while_no_timeout
    done
}
# vuln-code-snippet end dos_while_no_timeout

# vuln-code-snippet start dos_recursive_glob
process_user_pattern() {
    local glob_pattern="$1"
    # User-controlled glob — "/**/*" triggers recursive expansion across
    # entire filesystem, consuming CPU and memory for pathname generation.
    local files=($glob_pattern)  # vuln-code-snippet vuln-line dos_recursive_glob
    echo "Processing ${#files[@]} files"
}
# vuln-code-snippet end dos_recursive_glob

# vuln-code-snippet start dos_unbounded_write
log_stream_to_disk() {
    local source="$1"
    # Pipes unlimited data from user-controlled source to disk.
    # No size cap — fills partition until ENOSPC.
    cat "$source" >> /var/log/stream.log  # vuln-code-snippet vuln-line dos_unbounded_write
}
# vuln-code-snippet end dos_unbounded_write

# --- Safe variants ---

# vuln-code-snippet start dos_bounded_loop
repeat_limited() {
    local count="$1"
    local cmd="$2"
    local MAX_ITERATIONS=1000
    # Cap user input to a safe maximum.
    if (( count > MAX_ITERATIONS )); then
        count=$MAX_ITERATIONS
    fi
    local i
    for ((i = 0; i < count; i++)); do
        "$cmd"  # vuln-code-snippet safe-line dos_bounded_loop
    done
}
# vuln-code-snippet end dos_bounded_loop

# vuln-code-snippet start dos_ulimit_protected
run_with_limits() {
    local cmd="$1"
    # ulimit restricts the subprocess: 100 MB virtual memory,
    # 256 open files, 50 max processes.
    (
        ulimit -v 102400
        ulimit -n 256
        ulimit -u 50
        eval "$cmd"  # vuln-code-snippet safe-line dos_ulimit_protected
    )
}
# vuln-code-snippet end dos_ulimit_protected

# vuln-code-snippet start dos_timeout_wrapper
fetch_with_timeout() {
    local url="$1"
    # timeout(1) kills the process after 30 seconds — prevents hanging.
    timeout 30 curl -sf "$url"  # vuln-code-snippet safe-line dos_timeout_wrapper
}
# vuln-code-snippet end dos_timeout_wrapper

# vuln-code-snippet start dos_cgroup_limit
run_cgroup_constrained() {
    local cmd="$1"
    # systemd-run creates a transient cgroup with memory and CPU limits.
    systemd-run --scope -p MemoryMax=512M -p CPUQuota=50% $cmd  # vuln-code-snippet safe-line dos_cgroup_limit
}
# vuln-code-snippet end dos_cgroup_limit

# vuln-code-snippet start dos_hardcoded_parallelism
parallel_process_safe() {
    local input_file="$1"
    # Fixed parallelism (4 workers) regardless of input size.
    cat "$input_file" | xargs -P 4 -I {} /usr/local/bin/process {}  # vuln-code-snippet safe-line dos_hardcoded_parallelism
}
# vuln-code-snippet end dos_hardcoded_parallelism

# vuln-code-snippet start dos_maxdepth_find
search_bounded() {
    local search_root="$1"
    local pattern="$2"
    # -maxdepth 3 limits recursion — prevents full filesystem traversal.
    find "$search_root" -maxdepth 3 -name "$pattern" -type f  # vuln-code-snippet safe-line dos_maxdepth_find
}
# vuln-code-snippet end dos_maxdepth_find

# vuln-code-snippet start dos_read_with_limit
ingest_limited() {
    local input_file="$1"
    local MAX_LINES=10000
    # head -n caps input — prevents memory exhaustion.
    local lines=()
    while IFS= read -r line; do
        lines+=("$line")
    done < <(head -n "$MAX_LINES" "$input_file")  # vuln-code-snippet safe-line dos_read_with_limit
}
# vuln-code-snippet end dos_read_with_limit

# vuln-code-snippet start dos_disk_quota_check
write_with_quota() {
    local source="$1"
    local dest="$2"
    local MAX_SIZE_MB=100
    # Check available space and file size before writing.
    local size_kb
    size_kb=$(du -k "$source" | cut -f1)
    if (( size_kb > MAX_SIZE_MB * 1024 )); then
        echo "File exceeds ${MAX_SIZE_MB}MB limit" >&2
        return 1
    fi
    cp "$source" "$dest"  # vuln-code-snippet safe-line dos_disk_quota_check
}
# vuln-code-snippet end dos_disk_quota_check

# vuln-code-snippet start dos_rate_limited_retry
fetch_with_backoff() {
    local url="$1"
    local MAX_RETRIES=5
    local attempt=0
    # Bounded retries with exponential backoff — prevents infinite loop.
    while (( attempt < MAX_RETRIES )); do
        if curl -sf "$url" > /dev/null; then
            return 0  # vuln-code-snippet safe-line dos_rate_limited_retry
        fi
        (( attempt++ ))
        sleep $(( 2 ** attempt ))
    done
    return 1
}
# vuln-code-snippet end dos_rate_limited_retry

# vuln-code-snippet start dos_tmpfs_size_limit
create_bounded_scratch() {
    local work_dir
    work_dir=$(mktemp -d)
    # Mount tmpfs with size limit — kernel enforces the cap.
    sudo mount -t tmpfs -o size=100M tmpfs "$work_dir"  # vuln-code-snippet safe-line dos_tmpfs_size_limit
    echo "$work_dir"
}
# vuln-code-snippet end dos_tmpfs_size_limit

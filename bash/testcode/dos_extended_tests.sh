#!/bin/bash
# Denial of Service Extended Test Cases (CWE-770)
# Additional patterns beyond v0.4.0: unbounded parallelism, decompression bombs,
# resource-unlimited loops, recursive copies, and extended safe variants.

# vuln-code-snippet start dos_parallel_unbounded
process_items_parallel() {
    local count="$1"
    # -P "$count" spawns as many parallel processes as the user specifies. A user
    # supplying count=10000 creates 10,000 concurrent worker processes, exhausting
    # process table entries and memory. xargs parallelism must be bounded to a fixed
    # constant.
    seq 1 "$count" | xargs -P "$count" -I{} process_item {}  # vuln-code-snippet vuln-line dos_parallel_unbounded
}
# vuln-code-snippet end dos_parallel_unbounded

# vuln-code-snippet start dos_yes_user_bytes
fill_buffer() {
    local nbytes="$1"
    # yes generates infinite output; head -c "$nbytes" consumes nbytes bytes. With
    # user-controlled nbytes set to a large value (e.g., 1TB), this drives CPU at
    # 100% consuming the pipe buffer until head terminates — or memory exhausts if
    # redirected to a file instead of /dev/null.
    yes | head -c "$nbytes" > /dev/null  # vuln-code-snippet vuln-line dos_yes_user_bytes
}
# vuln-code-snippet end dos_yes_user_bytes

# vuln-code-snippet start dos_find_compress_all
compress_all_logs() {
    # Searching from / with no -maxdepth limit traverses the entire filesystem,
    # potentially crossing NFS mounts, proc entries, and device files. Each match
    # triggers gzip compression. On large systems this can run for hours, starving
    # I/O and disk space.
    find / -name "*.log" -exec gzip -f {} \;  # vuln-code-snippet vuln-line dos_find_compress_all
}
# vuln-code-snippet end dos_find_compress_all

# vuln-code-snippet start dos_unzip_user_archive
extract_uploaded_archive() {
    local archive_path="$1"
    # No size check before extraction. A zip bomb (e.g., 42.zip: 42KB -> 4.5PB
    # uncompressed) exhausts disk space. Without ulimit -f, the extraction runs
    # until disk full, potentially crashing services that depend on /tmp having
    # free space.
    unzip "$archive_path" -d /tmp/extracted/  # vuln-code-snippet vuln-line dos_unzip_user_archive
}
# vuln-code-snippet end dos_unzip_user_archive

# vuln-code-snippet start dos_sort_user_file
sort_data_file() {
    local user_file="$1"
    # sort loads the entire input into memory (or uses disk-based mergesort for
    # large files). A user supplying a multi-gigabyte file exhausts available memory
    # and/or disk space in the tmp sort scratch area. No size validation is performed
    # before the sort operation.
    sort "$user_file" > /tmp/sorted_output.txt  # vuln-code-snippet vuln-line dos_sort_user_file
}
# vuln-code-snippet end dos_sort_user_file

# vuln-code-snippet start dos_curl_gunzip_pipe
download_and_count() {
    local url="$1"
    # A decompression bomb served at the URL expands to a massive uncompressed
    # stream. gunzip has no size limit; wc -l must read the entire stream. A 1MB
    # compressed file can expand to gigabytes, exhausting memory in the pipeline
    # buffer.
    curl -s "$url" | gunzip | wc -l  # vuln-code-snippet vuln-line dos_curl_gunzip_pipe
}
# vuln-code-snippet end dos_curl_gunzip_pipe

# vuln-code-snippet start dos_retry_no_limit
retry_command() {
    local cmd="$1"
    # No retry limit, no backoff, no timeout. If cmd consistently fails (service
    # down, always returns non-zero), this loop runs forever, consuming 100% CPU.
    # Even a brief availability issue causes the process to spin indefinitely.
    while ! "$cmd"; do  # vuln-code-snippet vuln-line dos_retry_no_limit
        true
    done
}
# vuln-code-snippet end dos_retry_no_limit

# vuln-code-snippet start dos_inotify_unthrottled
watch_directory_events() {
    local watch_dir="$1"
    # No rate limiting on the event consumer. A user who can create files in
    # watch_dir rapidly (e.g., a fork bomb creating empty files) generates millions
    # of inotify events per second, overwhelming process_file_event and starving
    # the event queue.
    inotifywait -mrq "$watch_dir" --format '%e %f' |  # vuln-code-snippet vuln-line dos_inotify_unthrottled
        while IFS=' ' read -r event filepath; do
            process_file_event "$event" "$filepath"
        done
}
# vuln-code-snippet end dos_inotify_unthrottled

# vuln-code-snippet start dos_cp_recursive_user
stage_user_content() {
    local user_src="$1"
    # The user controls the source path. Supplying a large directory tree (or a
    # symlink pointing to a deep directory structure) exhausts disk space in
    # /tmp/staging. No size check or depth limit is applied before the recursive
    # copy begins.
    cp -r "$user_src" /tmp/staging/  # vuln-code-snippet vuln-line dos_cp_recursive_user
}
# vuln-code-snippet end dos_cp_recursive_user

# vuln-code-snippet start dos_wget_recursive
mirror_website() {
    local url="$1"
    # -r enables recursive download; -l 10 limits depth to 10 levels. A site with
    # wide branching can still yield millions of files. User control of the URL
    # enables targeting large sites or internal services with many endpoints,
    # exhausting disk space.
    wget -r -l 10 --no-parent "$url" -P /tmp/mirror/  # vuln-code-snippet vuln-line dos_wget_recursive
}
# vuln-code-snippet end dos_wget_recursive

# vuln-code-snippet start dos_mkfifo_spin
start_pipe_processor() {
    mkfifo /tmp/event_pipe
    # The FIFO loop has no rate limiting. If a producer writes to the FIFO at high
    # speed, the while loop consumes 100% CPU processing events without throttling.
    # With no termination condition other than external signal, the loop runs
    # indefinitely.
    while true; do  # vuln-code-snippet vuln-line dos_mkfifo_spin
        read -r line < /tmp/event_pipe
        echo "$line"
    done
}
# vuln-code-snippet end dos_mkfifo_spin

# vuln-code-snippet start dos_rsync_delete_user
sync_user_directory() {
    local user_src="$1"
    local dest="$2"
    # --delete removes files in dest that don't exist in src. With user-controlled
    # user_src, the attacker can empty the destination by supplying an empty
    # directory. --recursive combined with a large source tree exhausts disk I/O.
    rsync --recursive --delete "$user_src" "$dest"  # vuln-code-snippet vuln-line dos_rsync_delete_user
}
# vuln-code-snippet end dos_rsync_delete_user

# vuln-code-snippet start dos_python_string_mult
generate_padding() {
    local user_input="$1"
    # String multiplication by 1,000,000 allocates len(user_input) * 1,000,000
    # bytes of memory immediately. A 1KB input becomes 1GB. No memory limit is
    # imposed before the multiplication, potentially causing OOM-killer to terminate
    # critical processes.
    python3 -c "x = '${user_input}' * 1000000; print(len(x))"  # vuln-code-snippet vuln-line dos_python_string_mult
}
# vuln-code-snippet end dos_python_string_mult

# vuln-code-snippet start dos_dd_user_size_input
read_device_data() {
    local device_path="$1"
    # No count limit — dd reads the entire device. User-supplied device_path can
    # be /dev/sda (entire disk) or a loop device backed by a large file. Without
    # count=N, dd reads until EOF, potentially for hours while consuming I/O
    # bandwidth.
    dd if="$device_path" bs=1M > /dev/null  # vuln-code-snippet vuln-line dos_dd_user_size_input
}
# vuln-code-snippet end dos_dd_user_size_input

# vuln-code-snippet start dos_pv_rate_user
stream_at_rate() {
    local rate_mbps="$1"
    # rate_mbps is user-controlled. Supplying 10000 (10Gbps) saturates the CPU
    # pipeline processing loop at maximum speed, effectively becoming an unbounded
    # resource consumer. Even with pv rate limiting, the rate itself is not
    # validated.
    cat /dev/zero | pv -q -L "${rate_mbps}m" > /dev/null  # vuln-code-snippet vuln-line dos_pv_rate_user
}
# vuln-code-snippet end dos_pv_rate_user

# --- Safe variants ---

# vuln-code-snippet start dos_bounded_seq_loop
run_fixed_iterations() {
    # The loop bound (10) is a hardcoded constant. No user input controls the
    # iteration count. This is a discrimination test: tools flagging all loops
    # without checking whether the bound is user-controlled will FP here.
    for i in $(seq 1 10); do  # vuln-code-snippet safe-line dos_bounded_seq_loop
        process_item "$i"
    done
}
# vuln-code-snippet end dos_bounded_seq_loop

# vuln-code-snippet start dos_find_maxdepth_filter
clean_old_logs() {
    # -maxdepth 2 limits directory depth; -mtime +30 limits matches to old files.
    # Both constraints are hardcoded. The combination prevents unbounded filesystem
    # traversal.
    find /var/log/app -maxdepth 2 -name "*.log" -mtime +30 -delete  # vuln-code-snippet safe-line dos_find_maxdepth_filter
}
# vuln-code-snippet end dos_find_maxdepth_filter

# vuln-code-snippet start dos_dd_hardcoded_count
create_test_image() {
    # count=100 is hardcoded — the output is bounded to exactly 100MB. No user
    # input controls the count or size parameter.
    dd if=/dev/zero bs=1M count=100 of=/tmp/test.img  # vuln-code-snippet safe-line dos_dd_hardcoded_count
}
# vuln-code-snippet end dos_dd_hardcoded_count

# vuln-code-snippet start dos_ulimit_before_tar
extract_with_limit() {
    local archive="$1"
    ulimit -f 102400
    # ulimit -f 102400 sets a 100MB file size limit for the current process. Any
    # file extraction attempt that creates a file larger than 100MB receives SIGXFSZ
    # (file size exceeded), preventing zip bomb decompression.
    tar xf "$archive" -C /tmp/work/  # vuln-code-snippet safe-line dos_ulimit_before_tar
}
# vuln-code-snippet end dos_ulimit_before_tar

# vuln-code-snippet start dos_timeout_cmd
run_with_timeout() {
    local cmd="$1"
    # timeout 30 sends SIGTERM after 30 seconds regardless of what cmd is doing.
    # No loop or computation can run indefinitely under this wrapper. The 30-second
    # bound is hardcoded.
    timeout 30 "$cmd"  # vuln-code-snippet safe-line dos_timeout_cmd
}
# vuln-code-snippet end dos_timeout_cmd

# vuln-code-snippet start dos_head_line_limit
read_log_sample() {
    local user_file="$1"
    # head -n 1000 reads at most 1000 lines regardless of file size. A user
    # supplying a multi-gigabyte file causes head to read only the first 1000 lines
    # then exit. Memory consumption is bounded by line length x 1000.
    head -n 1000 "$user_file"  # vuln-code-snippet safe-line dos_head_line_limit
}
# vuln-code-snippet end dos_head_line_limit

# vuln-code-snippet start dos_xargs_bounded_parallel
process_items_bounded() {
    # -P 4 is a hardcoded maximum of 4 parallel processes. The degree of
    # parallelism does not scale with input size. -n 1 ensures each process handles
    # one item at a time.
    xargs -P 4 -n 1 process_item  # vuln-code-snippet safe-line dos_xargs_bounded_parallel
}
# vuln-code-snippet end dos_xargs_bounded_parallel

# vuln-code-snippet start dos_read_timeout_limit
read_user_input() {
    # -t 10 imposes a 10-second timeout — read exits with non-zero status if no
    # input arrives. -n 256 limits input to 256 characters. Both time and data
    # size are bounded.
    read -t 10 -n 256 -r user_input  # vuln-code-snippet safe-line dos_read_timeout_limit
}
# vuln-code-snippet end dos_read_timeout_limit

# vuln-code-snippet start dos_while_bounded_head
process_file_lines() {
    local user_file="$1"
    # The while loop iterates over head's bounded output. head -n 10000 caps input
    # at 10,000 lines regardless of user_file size. The loop terminates when
    # head's output is exhausted.
    while IFS= read -r line; do  # vuln-code-snippet safe-line dos_while_bounded_head
        process_line "$line"
    done < <(head -n 10000 "$user_file")
}
# vuln-code-snippet end dos_while_bounded_head

# vuln-code-snippet start dos_find_user_maxdepth1
list_user_files() {
    local user_dir="$1"
    # -maxdepth 1 prevents recursive directory traversal. Even when user_dir is
    # user-controlled, find only lists files in the immediate directory level, not
    # nested subdirectories.
    find "$user_dir" -maxdepth 1 -type f -name "*.csv"  # vuln-code-snippet safe-line dos_find_user_maxdepth1
}
# vuln-code-snippet end dos_find_user_maxdepth1

# vuln-code-snippet start dos_pv_rate_limited
rate_limited_transfer() {
    local user_file="$1"
    # -L 10m hard-limits throughput to 10 megabytes per second regardless of
    # user_file size. The rate is a hardcoded constant, not user-controlled.
    pv -q -L 10m "$user_file" | process_stream  # vuln-code-snippet safe-line dos_pv_rate_limited
}
# vuln-code-snippet end dos_pv_rate_limited

# vuln-code-snippet start dos_parallel_jobs_timeout
batch_process() {
    # --jobs 4 bounds concurrency; --timeout 60 kills any job running longer than
    # 60 seconds. Both limits are hardcoded constants.
    parallel --jobs 4 --timeout 60 process_item ::: "${BATCH_ITEMS[@]}"  # vuln-code-snippet safe-line dos_parallel_jobs_timeout
}
# vuln-code-snippet end dos_parallel_jobs_timeout

# vuln-code-snippet start dos_truncate_bounded
create_scratch_file() {
    # truncate -s 100M creates a file of exactly 100MB (as a sparse file on most
    # filesystems). The size is a hardcoded constant, not user-controlled.
    truncate -s 100M /tmp/scratch.img  # vuln-code-snippet safe-line dos_truncate_bounded
}
# vuln-code-snippet end dos_truncate_bounded

# vuln-code-snippet start dos_cgroup_wrapped
run_resource_limited() {
    local cmd="$1"
    # systemd cgroup limits bound memory to 512MB and CPU to 50% regardless of
    # what cmd does. The limits are enforced by the kernel's cgroup v2 interface
    # and cannot be escaped by the child process.
    systemd-run --scope -p MemoryMax=512M -p CPUQuota=50% "$cmd"  # vuln-code-snippet safe-line dos_cgroup_wrapped
}
# vuln-code-snippet end dos_cgroup_wrapped

# vuln-code-snippet start dos_tail_bounded
monitor_log_bounded() {
    local log_file="$1"
    # head -n 10000 terminates the pipeline after reading 10,000 lines. Regardless
    # of how long tail -f runs or how fast log_file grows, the pipeline exits after
    # consuming exactly 10,000 lines.
    tail -f "$log_file" | head -n 10000  # vuln-code-snippet safe-line dos_tail_bounded
}
# vuln-code-snippet end dos_tail_bounded

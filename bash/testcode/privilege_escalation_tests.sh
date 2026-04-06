#!/bin/bash
# Privilege Escalation Test Cases (CWE-250)
# Execution with unnecessary privileges: sudo/su/docker --privileged
# with user-controlled arguments. Distinct from cmdi (CWE-78): cmdi tests
# whether user input reaches a command execution sink; this category tests
# whether a command runs with elevated privileges it does not need.

# vuln-code-snippet start privesc_sudo_user_command
run_admin_task() {
    local task_cmd="$1"
    # User-controlled $task_cmd executed as root. Attacker supplies
    # "chmod 777 /etc/shadow" or any arbitrary command.
    sudo $task_cmd  # vuln-code-snippet vuln-line privesc_sudo_user_command
}
# vuln-code-snippet end privesc_sudo_user_command

# vuln-code-snippet start privesc_docker_privileged
run_user_container() {
    local image="$1"
    # --privileged disables all container security (AppArmor, seccomp,
    # capabilities). User-controlled $image with full host access.
    docker run --privileged "$image"  # vuln-code-snippet vuln-line privesc_docker_privileged
}
# vuln-code-snippet end privesc_docker_privileged

# vuln-code-snippet start privesc_su_user_shell
switch_and_execute() {
    local target_user="$1"
    local command="$2"
    # su -c with user-controlled command AND target user.
    su - "$target_user" -c "$command"  # vuln-code-snippet vuln-line privesc_su_user_shell
}
# vuln-code-snippet end privesc_su_user_shell

# vuln-code-snippet start privesc_nsenter_target
debug_container_ns() {
    local container_pid="$1"
    # nsenter with --target from user input enters any PID namespace.
    # Attacker supplies PID 1 to enter the host's init namespace.
    nsenter --target "$container_pid" --mount --uts --ipc --net --pid -- /bin/bash  # vuln-code-snippet vuln-line privesc_nsenter_target
}
# vuln-code-snippet end privesc_nsenter_target

# vuln-code-snippet start privesc_setcap_user_binary
grant_capabilities() {
    local binary_path="$1"
    # setcap with user-controlled path — attacker grants cap_net_raw
    # (packet sniffing) or cap_sys_admin (mount, bpf) to any binary.
    setcap cap_net_raw+ep "$binary_path"  # vuln-code-snippet vuln-line privesc_setcap_user_binary
}
# vuln-code-snippet end privesc_setcap_user_binary

# vuln-code-snippet start privesc_pkexec_user_cmd
run_polkit_action() {
    local action_cmd="$1"
    # pkexec runs $action_cmd as root via PolicyKit — user controls
    # the command entirely.
    pkexec $action_cmd  # vuln-code-snippet vuln-line privesc_pkexec_user_cmd
}
# vuln-code-snippet end privesc_pkexec_user_cmd

# vuln-code-snippet start privesc_chown_root_user_path
take_ownership() {
    local target_path="$1"
    # chown root on user-controlled path — attacker supplies /etc/shadow
    # or a symlink to any file, gaining root ownership.
    sudo chown root:root "$target_path"  # vuln-code-snippet vuln-line privesc_chown_root_user_path
}
# vuln-code-snippet end privesc_chown_root_user_path

# vuln-code-snippet start privesc_sudoers_write
add_sudoers_rule() {
    local username="$1"
    # Writing user-controlled $username into sudoers without validation.
    # Attacker supplies "ALL ALL=(ALL) NOPASSWD: ALL #" to grant full sudo.
    echo "${username} ALL=(ALL) NOPASSWD: /usr/bin/systemctl" >> /etc/sudoers  # vuln-code-snippet vuln-line privesc_sudoers_write
}
# vuln-code-snippet end privesc_sudoers_write

# vuln-code-snippet start privesc_docker_mount_root
run_container_host_mount() {
    local image="$1"
    # Mounting / into the container gives full read/write access to the
    # host filesystem — container escape via chroot or direct file modification.
    docker run -v /:/host "$image"  # vuln-code-snippet vuln-line privesc_docker_mount_root
}
# vuln-code-snippet end privesc_docker_mount_root

# vuln-code-snippet start privesc_chmod_suid
set_suid_bit() {
    local binary="$1"
    # Setting SUID on a user-controlled binary path. Attacker creates
    # a binary that spawns a root shell, then triggers this function.
    chmod u+s "$binary"  # vuln-code-snippet vuln-line privesc_chmod_suid
}
# vuln-code-snippet end privesc_chmod_suid

# --- Safe variants ---

# vuln-code-snippet start privesc_sudo_hardcoded
restart_service_safe() {
    # Hardcoded command — no user input reaches sudo.
    sudo systemctl restart nginx  # vuln-code-snippet safe-line privesc_sudo_hardcoded
}
# vuln-code-snippet end privesc_sudo_hardcoded

# vuln-code-snippet start privesc_docker_unprivileged
run_sandboxed_container() {
    local image="$1"
    # No --privileged, no host mounts, no extra capabilities.
    # Default Docker security profile applies.
    docker run --rm --read-only "$image"  # vuln-code-snippet safe-line privesc_docker_unprivileged
}
# vuln-code-snippet end privesc_docker_unprivileged

# vuln-code-snippet start privesc_capability_drop
run_minimal_caps() {
    local image="$1"
    # Drop ALL capabilities, only add the specific one needed.
    docker run --cap-drop=ALL --cap-add=NET_BIND_SERVICE "$image"  # vuln-code-snippet safe-line privesc_capability_drop
}
# vuln-code-snippet end privesc_capability_drop

# vuln-code-snippet start privesc_allowlist_command
run_allowed_admin_task() {
    local task_name="$1"
    # Only allow specific known commands — rejects arbitrary input.
    case "$task_name" in
        restart-nginx)  sudo systemctl restart nginx ;;
        restart-app)    sudo systemctl restart myapp ;;
        clear-cache)    sudo rm -rf /var/cache/myapp/* ;;
        *)
            echo "Unknown task: $task_name" >&2
            return 1  # vuln-code-snippet safe-line privesc_allowlist_command
            ;;
    esac
}
# vuln-code-snippet end privesc_allowlist_command

# vuln-code-snippet start privesc_runas_nobody
process_upload() {
    local file="$1"
    # Explicitly drops to nobody — minimal privileges for file processing.
    sudo -u nobody /usr/local/bin/process_upload "$file"  # vuln-code-snippet safe-line privesc_runas_nobody
}
# vuln-code-snippet end privesc_runas_nobody

# vuln-code-snippet start privesc_readonly_mount
run_with_readonly() {
    local image="$1"
    local data_dir="$2"
    # Read-only bind mount — container cannot modify host files.
    docker run --rm -v "${data_dir}:/data:ro" "$image"  # vuln-code-snippet safe-line privesc_readonly_mount
}
# vuln-code-snippet end privesc_readonly_mount

# vuln-code-snippet start privesc_sudo_specific_binary
update_system_time() {
    local ntp_server="$1"
    # sudo restricted to specific binary path — even if $ntp_server is
    # attacker-controlled, only ntpdate can execute.
    sudo /usr/sbin/ntpdate "$ntp_server"  # vuln-code-snippet safe-line privesc_sudo_specific_binary
}
# vuln-code-snippet end privesc_sudo_specific_binary

# vuln-code-snippet start privesc_no_new_privileges
run_restricted_container() {
    local image="$1"
    # --security-opt=no-new-privileges prevents SUID/SGID escalation
    # inside the container. Processes cannot gain new privileges.
    docker run --rm --security-opt=no-new-privileges "$image"  # vuln-code-snippet safe-line privesc_no_new_privileges
}
# vuln-code-snippet end privesc_no_new_privileges

# vuln-code-snippet start privesc_validate_sudoers
add_sudoers_validated() {
    local username="$1"
    # Validate username is alphanumeric only — prevents injection of
    # sudoers directives via special characters.
    if [[ ! "$username" =~ ^[a-z_][a-z0-9_-]*$ ]]; then
        echo "Invalid username format" >&2
        return 1
    fi
    echo "${username} ALL=(ALL) NOPASSWD: /usr/bin/systemctl restart myapp" | \
        sudo tee /etc/sudoers.d/"$username" > /dev/null  # vuln-code-snippet safe-line privesc_validate_sudoers
    sudo visudo -cf /etc/sudoers.d/"$username"
}
# vuln-code-snippet end privesc_validate_sudoers

# vuln-code-snippet start privesc_rootless_container
run_rootless() {
    local image="$1"
    # Podman rootless — entire container runtime runs as non-root.
    # No kernel-level privilege escalation possible.
    podman run --rm "$image"  # vuln-code-snippet safe-line privesc_rootless_container
}
# vuln-code-snippet end privesc_rootless_container

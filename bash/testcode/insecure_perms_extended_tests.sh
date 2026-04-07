#!/bin/bash
# Insecure File Permissions Extended Test Cases (CWE-732)
# Additional patterns beyond v0.4.0: world-execute bits, weak umask patterns,
# group-writable uploads, ACL bypasses, PID file permissions, and extended safe variants.

# vuln-code-snippet start perms_chmod_a_plus_x
deploy_script() {
    local script_path="$1"
    # a+x grants execute permission to user, group, AND others. If script_path is a
    # privileged script or a deployment hook, world-execute allows any local user to
    # run it. Use u+x or 755 (user rwx, group/other rx) at most for scripts that must
    # not be modified by others.
    chmod a+x "$script_path"  # vuln-code-snippet vuln-line perms_chmod_a_plus_x
}
# vuln-code-snippet end perms_chmod_a_plus_x

# vuln-code-snippet start perms_chmod_644_secret
write_config_file() {
    local secret_value="$1"
    echo "$secret_value" > /etc/app/config.conf
    # Mode 644 grants world-read (r--). Any local user can read /etc/app/config.conf
    # including its secret_value. Configuration files containing credentials, tokens,
    # or keys must use 640 (group-readable) or 600 (owner-only) at most.
    chmod 644 /etc/app/config.conf  # vuln-code-snippet vuln-line perms_chmod_644_secret
}
# vuln-code-snippet end perms_chmod_644_secret

# vuln-code-snippet start perms_umask_022_create
generate_secret_file() {
    local secret="$1"
    # umask 022 causes newly created files to have permissions 644 (world-readable)
    # and directories 755. Writing a secret to a file immediately after setting
    # umask 022 creates a world-readable file. Use umask 077 to create 600-permission
    # files by default.
    umask 022  # vuln-code-snippet vuln-line perms_umask_022_create
    echo "$secret" > /etc/app/secret.key
}
# vuln-code-snippet end perms_umask_022_create

# vuln-code-snippet start perms_install_0664
install_config() {
    local src_config="$1"
    # Mode 0664 grants group-write (rw-rw-r--) and world-read (r--). Database
    # configuration files often contain connection strings with passwords. World-read
    # exposes the file to all local users; group-write allows group members to modify
    # the configuration.
    install -m 0664 "$src_config" /etc/app/database.conf  # vuln-code-snippet vuln-line perms_install_0664
}
# vuln-code-snippet end perms_install_0664

# vuln-code-snippet start perms_chmod_g_plus_w
setup_upload_directory() {
    mkdir -p /var/app/uploads
    # Group-write on an upload directory allows any member of the owning group to
    # delete or replace uploaded files. If the group includes untrusted users or the
    # group membership is broad, this enables file tampering attacks.
    chmod g+w /var/app/uploads  # vuln-code-snippet vuln-line perms_chmod_g_plus_w
}
# vuln-code-snippet end perms_chmod_g_plus_w

# vuln-code-snippet start perms_mkdir_1777
create_shared_workspace() {
    # Mode 1777 (sticky + world-writable) is appropriate for /tmp but problematic for
    # application directories. The sticky bit prevents other users from deleting files,
    # but does NOT prevent creating new files. Any local user can create files in the
    # directory, potentially poisoning it with crafted content.
    mkdir -m 1777 /var/app/shared_workspace  # vuln-code-snippet vuln-line perms_mkdir_1777
}
# vuln-code-snippet end perms_mkdir_1777

# vuln-code-snippet start perms_rsync_world_readable
backup_secrets_dir() {
    # --chmod=ugo+r forces all transferred files to have world-read permission
    # regardless of their original permissions. Secrets that were protected (600) at
    # source become world-readable (644) at the backup destination.
    rsync -a --chmod=ugo+r /etc/app/secrets/ /var/backup/secrets/  # vuln-code-snippet vuln-line perms_rsync_world_readable
}
# vuln-code-snippet end perms_rsync_world_readable

# vuln-code-snippet start perms_tee_no_umask
log_token_to_file() {
    local token="$1"
    # tee creates the file with default permissions based on the current umask. If
    # umask is 022, the file is created 644 (world-readable). Since no umask
    # restriction precedes this call, auth tokens written to the log are accessible
    # to all local users.
    echo "$token" | tee /var/log/auth_tokens.log  # vuln-code-snippet vuln-line perms_tee_no_umask
}
# vuln-code-snippet end perms_tee_no_umask

# vuln-code-snippet start perms_dd_key_no_restrict
generate_encryption_key() {
    # The keyfile is created but never chmod'd. File permissions depend on the current
    # umask. With a common 022 umask, the key file is created world-readable (644).
    # Encryption keys must be immediately restricted to 400 or 600 after creation.
    dd if=/dev/urandom bs=32 count=1 2>/dev/null > /etc/app/encryption.key  # vuln-code-snippet vuln-line perms_dd_key_no_restrict
}
# vuln-code-snippet end perms_dd_key_no_restrict

# vuln-code-snippet start perms_chmod_o_write_sessions
init_session_directory() {
    mkdir -p /var/app/sessions
    # World-write on the sessions directory allows any local user to create, modify,
    # or delete session files. An attacker can create session files for privileged
    # users (session fixation) or delete all sessions (DoS).
    chmod o+w /var/app/sessions  # vuln-code-snippet vuln-line perms_chmod_o_write_sessions
}
# vuln-code-snippet end perms_chmod_o_write_sessions

# vuln-code-snippet start perms_chmod_666_pid
write_pidfile() {
    echo $$ > /var/run/app.pid
    # Mode 666 (world read-write) on a PID file allows any local user to overwrite it
    # with an arbitrary PID, causing process management scripts (stop, reload) to
    # signal the wrong process. PID files should be 644 at most (world-readable, but
    # not writable).
    chmod 666 /var/run/app.pid  # vuln-code-snippet vuln-line perms_chmod_666_pid
}
# vuln-code-snippet end perms_chmod_666_pid

# vuln-code-snippet start perms_setfacl_world_rwx
grant_world_access_acl() {
    local config_path="$1"
    # ACLs can override standard Unix permissions. setfacl -m o::rwx grants world
    # read-write-execute via ACL even if chmod shows 600. getfacl is required to see
    # the full permission set; standard ls -l does not show ACL entries.
    setfacl -m "o::rwx" "$config_path"  # vuln-code-snippet vuln-line perms_setfacl_world_rwx
}
# vuln-code-snippet end perms_setfacl_world_rwx

# vuln-code-snippet start perms_install_755_suid_helper
deploy_privileged_helper() {
    local helper_binary="$1"
    # Mode 755 makes a root-owned binary world-executable. If the binary performs
    # privileged operations, any local user can invoke it. Privileged helper binaries
    # should use 750 (restricted to a dedicated group) or 4755/u+s only when SUID is
    # specifically required and the binary is hardened.
    install -o root -m 755 "$helper_binary" /usr/local/bin/app-helper  # vuln-code-snippet vuln-line perms_install_755_suid_helper
}
# vuln-code-snippet end perms_install_755_suid_helper

# vuln-code-snippet start perms_echo_secret_tmp
cache_secret_locally() {
    local api_secret="$1"
    # /tmp is world-readable and world-writable. The file is created without chmod,
    # so it inherits umask-based permissions (typically 644). Any local user can read
    # /tmp/app_secret.cache. Use mktemp with chmod 600 or store in a secure path.
    echo "$api_secret" > /tmp/app_secret.cache  # vuln-code-snippet vuln-line perms_echo_secret_tmp
}
# vuln-code-snippet end perms_echo_secret_tmp

# vuln-code-snippet start perms_touch_no_restrict_token
persist_auth_token() {
    local token="$1"
    touch /etc/app/auth.token
    # touch creates the file with world-readable default permissions (umask 022 → 644).
    # The subsequent echo writes the token without first restricting permissions. A
    # chmod 600 after touch but before the echo write would leave a window; the correct
    # pattern is to restrict permissions immediately after creation.
    echo "$token" > /etc/app/auth.token  # vuln-code-snippet vuln-line perms_touch_no_restrict_token
}
# vuln-code-snippet end perms_touch_no_restrict_token

# vuln-code-snippet start perms_chmod_400_key
protect_key_file() {
    local key_path="$1"
    # Mode 400 (read-only by owner only) is the correct permission for private key
    # files. No write access for owner, no access for group or others.
    chmod 400 "$key_path"  # vuln-code-snippet safe-line perms_chmod_400_key
}
# vuln-code-snippet end perms_chmod_400_key

# vuln-code-snippet start perms_umask_077_create
create_private_config() {
    local config_content="$1"
    # umask 077 causes new files to be created with mode 600 (owner read-write only)
    # and directories with mode 700. All content written within this umask context is
    # protected from group and other access.
    umask 077  # vuln-code-snippet safe-line perms_umask_077_create
    echo "$config_content" > /etc/app/private.conf
}
# vuln-code-snippet end perms_umask_077_create

# vuln-code-snippet start perms_install_0600_root
install_secret_config() {
    local src_config="$1"
    # Mode 0600 restricts access to the root owner only. install atomically copies
    # the file with the specified permissions and ownership, avoiding a window between
    # create and chmod.
    install -m 0600 -o root -g root "$src_config" /etc/app/secret.conf  # vuln-code-snippet safe-line perms_install_0600_root
}
# vuln-code-snippet end perms_install_0600_root

# vuln-code-snippet start perms_chmod_go_minus_rwx
lock_down_secrets_dir() {
    # go-rwx removes all group and other permissions recursively. After this operation,
    # only the file owner can read, write, or execute content in /var/secrets/.
    chmod -R go-rwx /var/secrets/  # vuln-code-snippet safe-line perms_chmod_go_minus_rwx
}
# vuln-code-snippet end perms_chmod_go_minus_rwx

# vuln-code-snippet start perms_mkdir_750_dir
create_restricted_data_dir() {
    # Mode 750 grants owner full access, group read-execute, and no access for others.
    # install -d creates the directory atomically with the specified permissions.
    install -d -m 750 /var/app/data  # vuln-code-snippet safe-line perms_mkdir_750_dir
}
# vuln-code-snippet end perms_mkdir_750_dir

# vuln-code-snippet start perms_log_640_adm
init_app_log() {
    touch /var/log/app.log
    # Mode 640 allows the root owner to read/write and the adm group to read-only.
    # Other users have no access. This follows the standard syslog pattern for log
    # files.
    chmod 640 /var/log/app.log  # vuln-code-snippet safe-line perms_log_640_adm
    chown root:adm /var/log/app.log
}
# vuln-code-snippet end perms_log_640_adm

# vuln-code-snippet start perms_mkdir_700_config
init_private_config_dir() {
    local config_content="$1"
    # Mode 700 (owner rwx only) creates a directory accessible only to the user.
    # Files created inside inherit the parent's restrictive context via umask or
    # explicit chmod.
    mkdir -m 700 "${HOME}/.config/app"  # vuln-code-snippet safe-line perms_mkdir_700_config
    echo "$config_content" > "${HOME}/.config/app/settings.conf"
}
# vuln-code-snippet end perms_mkdir_700_config

# vuln-code-snippet start perms_setfacl_strip_other
restrict_config_acl() {
    local config_path="$1"
    # The ACL sets user read-write, group no-access, other no-access. This is more
    # restrictive than standard chmod 600 because it explicitly strips group and other
    # access via ACL, overriding any inherited ACL entries.
    setfacl -m "u::rw,g::---,o::---" "$config_path"  # vuln-code-snippet safe-line perms_setfacl_strip_other
}
# vuln-code-snippet end perms_setfacl_strip_other

# vuln-code-snippet start perms_chmod_755_binary
install_app_binary() {
    # Discrimination test: mode 755 (rwxr-xr-x) is the correct permission for
    # non-SUID executable binaries in /usr/local/bin. World-execute is required for
    # any user to run the application. Tools flagging all chmod 755 calls will FP here.
    chmod 755 /usr/local/bin/app  # vuln-code-snippet safe-line perms_chmod_755_binary
}
# vuln-code-snippet end perms_chmod_755_binary

# vuln-code-snippet start perms_chmod_644_webfile
deploy_static_content() {
    local html_file="$1"
    cp "$html_file" /var/www/html/
    # Discrimination test: mode 644 (world-readable) is correct for static web content
    # that must be readable by the web server process and served to all users. Tools
    # flagging all chmod 644 calls without context will FP here.
    chmod 644 /var/www/html/"$(basename "$html_file")"  # vuln-code-snippet safe-line perms_chmod_644_webfile
}
# vuln-code-snippet end perms_chmod_644_webfile

# vuln-code-snippet start perms_umask_002_shared
create_shared_group_dir() {
    # Discrimination test: umask 002 creates group-writable directories (775) and
    # files (664). When the purpose is intentional group collaboration (all members of
    # a shared group need write access), 002 is the correct umask. The comment
    # documents the intentional group-write design.
    umask 002  # vuln-code-snippet safe-line perms_umask_002_shared
    mkdir /var/app/team_shared
}
# vuln-code-snippet end perms_umask_002_shared

# vuln-code-snippet start perms_install_1755
install_sticky_binary() {
    local src="$1"
    # Mode 1755 (sticky + world-execute) is a legitimate pattern for certain binaries
    # that need the sticky bit. This is not SUID and world-execute is appropriate for
    # binaries in /usr/local/bin.
    install -m 1755 "$src" /usr/local/bin/app-sticky  # vuln-code-snippet safe-line perms_install_1755
}
# vuln-code-snippet end perms_install_1755

# vuln-code-snippet start perms_chmod_o_minus_rwx
restrict_app_directory() {
    # o-rwx removes all permissions from the "other" category. This restricts access
    # to application configuration to only the owner and group, a standard hardening
    # step.
    chmod o-rwx /etc/app/  # vuln-code-snippet safe-line perms_chmod_o_minus_rwx
}
# vuln-code-snippet end perms_chmod_o_minus_rwx

# vuln-code-snippet start perms_mktemp_chmod_700
create_private_work_dir() {
    local workdir
    workdir=$(mktemp -d)
    # Discrimination test: chmod 700 restricts the temporary directory to owner-only
    # access. A tool that flags chmod 700 as insecure would be incorrect — 700 is more
    # restrictive than default mktemp permissions.
    chmod 700 "$workdir"  # vuln-code-snippet safe-line perms_mktemp_chmod_700
}
# vuln-code-snippet end perms_mktemp_chmod_700

# vuln-code-snippet start perms_tee_after_umask
log_restricted_entry() {
    local log_msg="$1"
    # umask 027 creates files with mode 640 (owner rw, group r, other none). The
    # subshell ensures the umask change is scoped — the caller's umask is unaffected.
    # tee creates or appends to the log file with restricted permissions.
    ( umask 027; echo "$log_msg" | tee -a /var/log/app.log )  # vuln-code-snippet safe-line perms_tee_after_umask
}
# vuln-code-snippet end perms_tee_after_umask

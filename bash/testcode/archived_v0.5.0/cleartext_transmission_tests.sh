#!/bin/bash
# Cleartext Transmission of Sensitive Information Test Cases (CWE-319)
# Sending credentials, tokens, or sensitive data over unencrypted channels.
# Distinct from ssl_bypass (CWE-295): 295 = encrypted channel with disabled
# certificate validation; 319 = no encryption at all (HTTP, FTP, telnet).

# vuln-code-snippet start cleartext_http_credentials
fetch_with_basic_auth() {
    local user="$1"
    local pass="$2"
    # http:// sends Basic Auth header (base64-encoded user:pass) in
    # plaintext — visible to any network observer or proxy.
    curl -u "${user}:${pass}" "http://api.internal/v1/data"  # vuln-code-snippet vuln-line cleartext_http_credentials
}
# vuln-code-snippet end cleartext_http_credentials

# vuln-code-snippet start cleartext_ftp_upload
upload_backup() {
    local file="$1"
    local host="$2"
    # FTP transmits credentials AND data in cleartext. Password visible
    # on the wire during AUTH exchange.
    curl -T "$file" "ftp://backup_user:Backup2025@${host}/backups/"  # vuln-code-snippet vuln-line cleartext_ftp_upload
}
# vuln-code-snippet end cleartext_ftp_upload

# vuln-code-snippet start cleartext_telnet_session
manage_device() {
    local host="$1"
    local command="$2"
    # telnet sends everything in cleartext — credentials, commands,
    # and responses are all observable.
    echo "$command" | telnet "$host" 23  # vuln-code-snippet vuln-line cleartext_telnet_session
}
# vuln-code-snippet end cleartext_telnet_session

# vuln-code-snippet start cleartext_netcat_send
send_data_nc() {
    local host="$1"
    local port="$2"
    local data="$3"
    # nc sends raw unencrypted bytes — no TLS, no authentication.
    echo "$data" | nc "$host" "$port"  # vuln-code-snippet vuln-line cleartext_netcat_send
}
# vuln-code-snippet end cleartext_netcat_send

# vuln-code-snippet start cleartext_mysql_no_tls
query_database() {
    local host="$1"
    local query="$2"
    # mysql client without --ssl-mode sends credentials and query data
    # in cleartext over the network.
    mysql -h "$host" -u app -pSecret123 -e "$query" appdb  # vuln-code-snippet vuln-line cleartext_mysql_no_tls
}
# vuln-code-snippet end cleartext_mysql_no_tls

# vuln-code-snippet start cleartext_redis_no_tls
cache_operation() {
    local host="$1"
    local command="$2"
    # redis-cli without --tls sends AUTH password and all commands
    # in cleartext.
    redis-cli -h "$host" -a "${REDIS_PASSWORD}" $command  # vuln-code-snippet vuln-line cleartext_redis_no_tls
}
# vuln-code-snippet end cleartext_redis_no_tls

# vuln-code-snippet start cleartext_curl_http_token
call_api_http() {
    local endpoint="$1"
    local token="$2"
    # Bearer token sent over http:// — token visible to any network
    # observer between client and server.
    curl -H "Authorization: Bearer ${token}" "http://api.internal${endpoint}"  # vuln-code-snippet vuln-line cleartext_curl_http_token
}
# vuln-code-snippet end cleartext_curl_http_token

# vuln-code-snippet start cleartext_wget_http_auth
download_artifact() {
    local url="$1"
    local token="$2"
    # wget over http:// with auth header — cleartext transmission.
    wget --header="X-API-Key: ${token}" "$url" -O /tmp/artifact.tar.gz  # vuln-code-snippet vuln-line cleartext_wget_http_auth
}
# vuln-code-snippet end cleartext_wget_http_auth

# vuln-code-snippet start cleartext_smtp_plain
send_alert_email() {
    local recipient="$1"
    local body="$2"
    # msmtp without --tls sends SMTP credentials and email body in
    # cleartext. STARTTLS not enforced.
    echo "$body" | msmtp --host=mail.internal --port=25 \
        --auth=plain --user=alerts --password=AlertPass123 "$recipient"  # vuln-code-snippet vuln-line cleartext_smtp_plain
}
# vuln-code-snippet end cleartext_smtp_plain

# vuln-code-snippet start cleartext_rsync_no_ssh
sync_files_daemon() {
    local src="$1"
    local dest_host="$2"
    # rsync daemon mode (::) uses its own protocol without encryption.
    # Credentials in RSYNC_PASSWORD env and all file data are cleartext.
    RSYNC_PASSWORD=SyncPass rsync "$src" "rsync://${dest_host}/backups/"  # vuln-code-snippet vuln-line cleartext_rsync_no_ssh
}
# vuln-code-snippet end cleartext_rsync_no_ssh

# --- Safe variants ---

# vuln-code-snippet start cleartext_https_credentials
fetch_with_https_auth() {
    local user="$1"
    local pass="$2"
    # https:// encrypts the entire HTTP exchange including auth headers.
    curl -u "${user}:${pass}" "https://api.internal/v1/data"  # vuln-code-snippet safe-line cleartext_https_credentials
}
# vuln-code-snippet end cleartext_https_credentials

# vuln-code-snippet start cleartext_sftp_upload
upload_backup_encrypted() {
    local file="$1"
    local host="$2"
    # SFTP runs over SSH — credentials and data are encrypted.
    sftp "backup_user@${host}" <<< "put ${file} /backups/"  # vuln-code-snippet safe-line cleartext_sftp_upload
}
# vuln-code-snippet end cleartext_sftp_upload

# vuln-code-snippet start cleartext_ssh_session
manage_device_secure() {
    local host="$1"
    local command="$2"
    # SSH encrypts the full session — replaces telnet.
    ssh "admin@${host}" "$command"  # vuln-code-snippet safe-line cleartext_ssh_session
}
# vuln-code-snippet end cleartext_ssh_session

# vuln-code-snippet start cleartext_ncat_ssl
send_data_encrypted() {
    local host="$1"
    local port="$2"
    local data="$3"
    # ncat --ssl wraps the connection in TLS.
    echo "$data" | ncat --ssl "$host" "$port"  # vuln-code-snippet safe-line cleartext_ncat_ssl
}
# vuln-code-snippet end cleartext_ncat_ssl

# vuln-code-snippet start cleartext_mysql_ssl_required
query_database_secure() {
    local host="$1"
    local query="$2"
    # --ssl-mode=REQUIRED forces TLS — connection fails if server
    # does not support encryption.
    mysql -h "$host" --ssl-mode=REQUIRED -u app -e "$query" appdb  # vuln-code-snippet safe-line cleartext_mysql_ssl_required
}
# vuln-code-snippet end cleartext_mysql_ssl_required

# vuln-code-snippet start cleartext_redis_tls
cache_operation_secure() {
    local host="$1"
    local command="$2"
    # --tls enables TLS encryption for the redis connection.
    redis-cli -h "$host" --tls -a "${REDIS_PASSWORD}" $command  # vuln-code-snippet safe-line cleartext_redis_tls
}
# vuln-code-snippet end cleartext_redis_tls

# vuln-code-snippet start cleartext_curl_https_token
call_api_secure() {
    local endpoint="$1"
    local token="$2"
    # https:// — token encrypted in transit.
    curl -H "Authorization: Bearer ${token}" "https://api.internal${endpoint}"  # vuln-code-snippet safe-line cleartext_curl_https_token
}
# vuln-code-snippet end cleartext_curl_https_token

# vuln-code-snippet start cleartext_smtp_starttls
send_alert_encrypted() {
    local recipient="$1"
    local body="$2"
    # --tls=on enforces STARTTLS — connection fails if server does
    # not support encryption upgrade.
    echo "$body" | msmtp --host=mail.internal --port=587 \
        --tls=on --auth=plain --user=alerts "$recipient"  # vuln-code-snippet safe-line cleartext_smtp_starttls
}
# vuln-code-snippet end cleartext_smtp_starttls

# vuln-code-snippet start cleartext_rsync_ssh
sync_files_encrypted() {
    local src="$1"
    local dest_host="$2"
    # rsync over SSH (-e ssh) — all data encrypted in transit.
    rsync -az -e ssh "$src" "${dest_host}:/backups/"  # vuln-code-snippet safe-line cleartext_rsync_ssh
}
# vuln-code-snippet end cleartext_rsync_ssh

# vuln-code-snippet start cleartext_scp_transfer
transfer_secure() {
    local file="$1"
    local dest="$2"
    # scp runs over SSH — inherently encrypted.
    scp "$file" "$dest"  # vuln-code-snippet safe-line cleartext_scp_transfer
}
# vuln-code-snippet end cleartext_scp_transfer

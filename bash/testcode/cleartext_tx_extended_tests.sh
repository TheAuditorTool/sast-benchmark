#!/bin/bash
# Cleartext Transmission Extended Test Cases (CWE-319)
# Additional patterns beyond v0.4.0: LDAP, POP3/IMAP, SVN, MongoDB, AMQP,
# Kafka, InfluxDB, rsh, auth-over-HTTP, socat, scp cipher none.

# vuln-code-snippet start cleartext_ldap_query
query_directory() {
    local host="$1"
    local dn="$2"
    local pass="$3"
    # ldap:// uses unencrypted LDAP on port 389. The bind password -w "$pass"
    # is transmitted in cleartext, observable by anyone on the network path
    # between client and directory server. Use ldaps:// (port 636) for
    # encrypted LDAP.
    ldapsearch -H "ldap://${host}" -D "$dn" -w "$pass" -b "dc=example,dc=com" "(objectClass=user)"  # vuln-code-snippet vuln-line cleartext_ldap_query
}
# vuln-code-snippet end cleartext_ldap_query

# vuln-code-snippet start cleartext_pop3_login
retrieve_mail() {
    local host="$1"
    local user="$2"
    local pass="$3"
    # POP3 on port 110 transmits username and password in cleartext. Network
    # observers between mail client and server can capture the credentials.
    # Use POP3S (pop3s:// on port 995) for TLS-encrypted mail retrieval.
    curl "pop3://${host}" -u "${user}:${pass}"  # vuln-code-snippet vuln-line cleartext_pop3_login
}
# vuln-code-snippet end cleartext_pop3_login

# vuln-code-snippet start cleartext_imap_login
fetch_inbox() {
    local host="$1"
    local user="$2"
    local pass="$3"
    # IMAP on port 143 transmits login credentials in cleartext. All mail
    # folder names, message headers, and body content are also transmitted
    # unencrypted. Use imaps:// (port 993) for TLS-encrypted IMAP.
    curl "imap://${host}/INBOX" -u "${user}:${pass}"  # vuln-code-snippet vuln-line cleartext_imap_login
}
# vuln-code-snippet end cleartext_imap_login

# vuln-code-snippet start cleartext_svn_checkout
checkout_repository() {
    local host="$1"
    # svn:// uses the native SVN protocol on port 3690 without encryption.
    # Authentication credentials and all repository content are transmitted
    # in cleartext. Use svn+ssh:// for SSH-tunneled encrypted access.
    svn checkout "svn://${host}/project/trunk" ./project  # vuln-code-snippet vuln-line cleartext_svn_checkout
}
# vuln-code-snippet end cleartext_svn_checkout

# vuln-code-snippet start cleartext_mongodb_no_tls
connect_to_database() {
    local user="$1"
    local pass="$2"
    local host="$3"
    # Without TLS query parameters (tls=true, tlsCAFile), the MongoDB
    # connection is unencrypted. Credentials and all query/response data
    # transit the network in plaintext. Use mongodb+srv:// with tls=true
    # or specify --tls flag.
    mongosh "mongodb://${user}:${pass}@${host}:27017/appdb"  # vuln-code-snippet vuln-line cleartext_mongodb_no_tls
}
# vuln-code-snippet end cleartext_mongodb_no_tls

# vuln-code-snippet start cleartext_amqp_no_ssl
publish_message() {
    local user="$1"
    local pass="$2"
    local host="$3"
    local message="$4"
    # amqp:// (port 5672) is the unencrypted AMQP protocol. Credentials in
    # the URI and all message content transit the network without encryption.
    # Use amqps:// (port 5671) for TLS-protected AMQP.
    amqp-publish -u "amqp://${user}:${pass}@${host}/" -r "tasks" -b "$message"  # vuln-code-snippet vuln-line cleartext_amqp_no_ssl
}
# vuln-code-snippet end cleartext_amqp_no_ssl

# vuln-code-snippet start cleartext_kafka_plaintext
produce_kafka_event() {
    local host="$1"
    local message="$2"
    # The PLAINTEXT:// protocol listener transmits all Kafka messages without
    # encryption. Authentication credentials (if configured) and message
    # payload are observable on the network. Use SSL:// (port 9093) for
    # encrypted Kafka.
    echo "$message" | kafka-console-producer --bootstrap-server "PLAINTEXT://${host}:9092" --topic events  # vuln-code-snippet vuln-line cleartext_kafka_plaintext
}
# vuln-code-snippet end cleartext_kafka_plaintext

# vuln-code-snippet start cleartext_influx_no_ssl
write_metric() {
    local host="$1"
    local user="$2"
    local pass="$3"
    local measurement="$4"
    # The influx CLI without --ssl connects over unencrypted HTTP. Credentials
    # and time-series data are transmitted in cleartext. Use influx --ssl
    # --host "$host" for TLS-protected connections.
    influx -host "$host" -username "$user" -password "$pass" -execute "INSERT ${measurement}"  # vuln-code-snippet vuln-line cleartext_influx_no_ssl
}
# vuln-code-snippet end cleartext_influx_no_ssl

# vuln-code-snippet start cleartext_mysql_ssl_disabled
run_query() {
    local host="$1"
    local pass="$2"
    local query="$3"
    # --ssl-mode=DISABLED explicitly disables TLS negotiation. The MySQL
    # client will connect in cleartext even if the server advertises TLS
    # support. Credentials and query results transit the network unencrypted.
    mysql --ssl-mode=DISABLED -h "$host" -uapp -p"$pass" -e "$query" appdb  # vuln-code-snippet vuln-line cleartext_mysql_ssl_disabled
}
# vuln-code-snippet end cleartext_mysql_ssl_disabled

# vuln-code-snippet start cleartext_rsh_command
run_remote_cmd() {
    local host="$1"
    local cmd="$2"
    # rsh (remote shell) is the unencrypted predecessor to SSH. All data
    # including commands, output, and authentication (if any) are transmitted
    # in plaintext. rsh should be replaced with ssh in all cases.
    rsh "$host" "$cmd"  # vuln-code-snippet vuln-line cleartext_rsh_command
}
# vuln-code-snippet end cleartext_rsh_command

# vuln-code-snippet start cleartext_auth_over_http
authenticate_to_api() {
    local token="$1"
    # Bearer tokens transmitted over HTTP are exposed to network observers,
    # proxy logs, and server access logs. Authentication endpoints must use
    # HTTPS. A captured token grants full API access for its lifetime.
    curl -s "http://api.example.com/v1/auth" -H "Authorization: Bearer ${token}"  # vuln-code-snippet vuln-line cleartext_auth_over_http
}
# vuln-code-snippet end cleartext_auth_over_http

# vuln-code-snippet start cleartext_socat_cleartext_shell
open_reverse_shell() {
    local host="$1"
    local port="$2"
    # socat TCP (without OPENSSL) creates an unencrypted TCP connection.
    # Combined with an interactive bash shell, all commands and output are
    # visible to network observers. The connection itself is also bidirectional
    # with full shell access.
    socat "TCP:${host}:${port}" EXEC:"bash -i"  # vuln-code-snippet vuln-line cleartext_socat_cleartext_shell
}
# vuln-code-snippet end cleartext_socat_cleartext_shell

# vuln-code-snippet start cleartext_scp_cipher_none
transfer_file_no_crypto() {
    local host="$1"
    local remote_file="$2"
    # Cipher=none disables all encryption on the SCP/SSH transport. While the
    # SSH handshake still occurs, the bulk data transfer uses no cipher. File
    # contents and metadata are transmitted in cleartext over what appears to
    # be a secure channel.
    scp -o "Cipher=none" "user@${host}:${remote_file}" ./  # vuln-code-snippet vuln-line cleartext_scp_cipher_none
}
# vuln-code-snippet end cleartext_scp_cipher_none

# vuln-code-snippet start cleartext_clickhouse_nossl
query_analytics() {
    local host="$1"
    local pass="$2"
    local query="$3"
    # The clickhouse-client without --secure connects over unencrypted native
    # protocol (port 9000). Password and all query/result data transit in
    # cleartext. Use --secure to enable TLS.
    clickhouse-client --host "$host" --password "$pass" --query "$query"  # vuln-code-snippet vuln-line cleartext_clickhouse_nossl
}
# vuln-code-snippet end cleartext_clickhouse_nossl

# vuln-code-snippet start cleartext_nc_data_send
transmit_sensitive_data() {
    local host="$1"
    local port="$2"
    local sensitive_data="$3"
    # netcat sends data over a raw unencrypted TCP connection. The
    # sensitive_data parameter is transmitted in cleartext to the destination
    # host:port. Any network observer between sender and receiver can capture
    # the content.
    echo "$sensitive_data" | nc "$host" "$port"  # vuln-code-snippet vuln-line cleartext_nc_data_send
}
# vuln-code-snippet end cleartext_nc_data_send

# --- Safe variants ---

# vuln-code-snippet start cleartext_ldaps_query
query_directory_tls() {
    local host="$1"
    local dn="$2"
    local pass="$3"
    # ldaps:// uses LDAP over TLS on port 636. The bind password and all
    # directory query data are encrypted in transit.
    ldapsearch -H "ldaps://${host}" -D "$dn" -w "$pass" -b "dc=example,dc=com" "(objectClass=user)"  # vuln-code-snippet safe-line cleartext_ldaps_query
}
# vuln-code-snippet end cleartext_ldaps_query

# vuln-code-snippet start cleartext_imaps_login
fetch_inbox_tls() {
    local host="$1"
    local user="$2"
    local pass="$3"
    # imaps:// uses IMAP over TLS (port 993). Credentials and mail content
    # are encrypted in transit.
    curl "imaps://${host}/INBOX" -u "${user}:${pass}"  # vuln-code-snippet safe-line cleartext_imaps_login
}
# vuln-code-snippet end cleartext_imaps_login

# vuln-code-snippet start cleartext_mongodb_tls
connect_database_tls() {
    local host="$1"
    # mongodb+srv:// with tls=true enables TLS for the MongoDB connection.
    # tlsCAFile pins the trusted CA certificate.
    mongosh "mongodb+srv://${host}/appdb?tls=true&tlsCAFile=/etc/ssl/certs/ca.pem"  # vuln-code-snippet safe-line cleartext_mongodb_tls
}
# vuln-code-snippet end cleartext_mongodb_tls

# vuln-code-snippet start cleartext_mysql_ssl_required_mode
run_query_tls() {
    local host="$1"
    local pass="$2"
    local query="$3"
    # --ssl-mode=REQUIRED enforces TLS. The MySQL client refuses to connect
    # if the server cannot negotiate TLS, preventing silent fallback to
    # cleartext.
    mysql --ssl-mode=REQUIRED -h "$host" -uapp -p"$pass" -e "$query" appdb  # vuln-code-snippet safe-line cleartext_mysql_ssl_required_mode
}
# vuln-code-snippet end cleartext_mysql_ssl_required_mode

# vuln-code-snippet start cleartext_redis_tls_flag
cache_lookup() {
    local host="$1"
    local key="$2"
    # --tls enables TLS encryption for the Redis connection. --cacert pins
    # the CA certificate to prevent MitM attacks.
    redis-cli -h "$host" --tls --cacert /etc/ssl/certs/ca.pem GET "$key"  # vuln-code-snippet safe-line cleartext_redis_tls_flag
}
# vuln-code-snippet end cleartext_redis_tls_flag

# vuln-code-snippet start cleartext_kafka_ssl
produce_event_tls() {
    local host="$1"
    local message="$2"
    # SSL:// uses the encrypted Kafka listener on port 9093. All message
    # data is encrypted in transit.
    echo "$message" | kafka-console-producer --bootstrap-server "SSL://${host}:9093" --topic events  # vuln-code-snippet safe-line cleartext_kafka_ssl
}
# vuln-code-snippet end cleartext_kafka_ssl

# vuln-code-snippet start cleartext_influx_ssl
write_metric_tls() {
    local host="$1"
    local user="$2"
    local pass="$3"
    local measurement="$4"
    # --ssl enables TLS for the InfluxDB connection. Credentials and
    # time-series data are encrypted in transit.
    influx --ssl -host "$host" -username "$user" -password "$pass" -execute "INSERT ${measurement}"  # vuln-code-snippet safe-line cleartext_influx_ssl
}
# vuln-code-snippet end cleartext_influx_ssl

# vuln-code-snippet start cleartext_amqps_url
publish_message_tls() {
    local user="$1"
    local pass="$2"
    local host="$3"
    local message="$4"
    # amqps:// uses AMQP over TLS (port 5671). Credentials and message
    # payload are encrypted.
    amqp-publish -u "amqps://${user}:${pass}@${host}/" -r "tasks" -b "$message"  # vuln-code-snippet safe-line cleartext_amqps_url
}
# vuln-code-snippet end cleartext_amqps_url

# vuln-code-snippet start cleartext_ssh_tunnel_mysql
connect_db_via_tunnel() {
    local bastion="$1"
    local db_host="$2"
    local pass="$3"
    # The MySQL connection goes through an SSH tunnel (encrypted at the SSH
    # layer). The actual TCP to MySQL travels over localhost only, with the
    # encrypted SSH tunnel handling the remote leg.
    ssh -f -N -L "3307:${db_host}:3306" "tunnel@${bastion}"
    mysql -h 127.0.0.1 -P 3307 -uapp -p"$pass" appdb  # vuln-code-snippet safe-line cleartext_ssh_tunnel_mysql
}
# vuln-code-snippet end cleartext_ssh_tunnel_mysql

# vuln-code-snippet start cleartext_rsync_over_ssh
sync_data_encrypted() {
    local host="$1"
    local remote_path="$2"
    local keyfile="$3"
    # -e "ssh -i $keyfile" routes rsync traffic through an SSH tunnel. All
    # file data, metadata, and authentication are encrypted.
    rsync -avz -e "ssh -i ${keyfile}" "/local/data/" "user@${host}:${remote_path}/"  # vuln-code-snippet safe-line cleartext_rsync_over_ssh
}
# vuln-code-snippet end cleartext_rsync_over_ssh

# vuln-code-snippet start cleartext_https_auth_token
call_auth_api() {
    local token="$1"
    # HTTPS encrypts the authorization header in transit. This is the safe
    # counterpart to cleartext_auth_over_http — same pattern, different scheme.
    curl -s "https://api.example.com/v1/auth" -H "Authorization: Bearer ${token}"  # vuln-code-snippet safe-line cleartext_https_auth_token
}
# vuln-code-snippet end cleartext_https_auth_token

# vuln-code-snippet start cleartext_nc_localhost_only
local_ipc_send() {
    local message="$1"
    # Discrimination test: netcat to localhost (loopback interface) never
    # leaves the machine. Loopback traffic is not observable by external
    # network observers. Tools flagging all nc calls without checking the
    # destination will FP here.
    echo "$message" | nc localhost 8080  # vuln-code-snippet safe-line cleartext_nc_localhost_only
}
# vuln-code-snippet end cleartext_nc_localhost_only

# vuln-code-snippet start cleartext_socat_openssl
connect_tls_host() {
    local host="$1"
    # OPENSSL:host:443 uses TLS for the connection. verify=1 enables
    # certificate verification, preventing MitM attacks. This is the safe
    # counterpart to socat TCP (cleartext).
    socat "OPENSSL:${host}:443,verify=1" STDIO  # vuln-code-snippet safe-line cleartext_socat_openssl
}
# vuln-code-snippet end cleartext_socat_openssl

# vuln-code-snippet start cleartext_svn_ssh
checkout_repo_encrypted() {
    local host="$1"
    # svn+ssh:// tunnels SVN over SSH. All repository data and authentication
    # are encrypted. This is the safe counterpart to svn:// (cleartext).
    svn checkout "svn+ssh://${host}/project/trunk" ./project  # vuln-code-snippet safe-line cleartext_svn_ssh
}
# vuln-code-snippet end cleartext_svn_ssh

# vuln-code-snippet start cleartext_prometheus_localhost
scrape_local_metrics() {
    # Discrimination test: scraping Prometheus metrics from localhost is not a
    # cleartext transmission security concern. The loopback interface does not
    # expose data to external networks. Internal monitoring tool calls to
    # localhost should not be flagged as CWE-319.
    curl -s "http://localhost:9090/api/v1/query?query=up"  # vuln-code-snippet safe-line cleartext_prometheus_localhost
}
# vuln-code-snippet end cleartext_prometheus_localhost

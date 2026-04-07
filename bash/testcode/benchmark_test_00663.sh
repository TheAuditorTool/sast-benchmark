#!/bin/bash
write_db_config_from_env() {
    cat << EOF > /etc/myapp/database.conf
[database]
host = ${DB_HOST:-localhost}
port = ${DB_PORT:-5432}
user = ${DB_USER:-app}
password = ${DB_PASSWORD:?DB_PASSWORD must be set}
EOF
    chmod 600 /etc/myapp/database.conf
}

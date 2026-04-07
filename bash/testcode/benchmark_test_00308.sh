#!/bin/bash
write_db_config() {
    cat << EOF > /etc/myapp/database.conf
[database]
host = db.production.internal
port = 5432
user = app_admin
password = Pr0duction_S3cret_2025!
EOF
    chmod 600 /etc/myapp/database.conf
}

#!/bin/bash
generate_env_file() {
    cat > .env << 'EOF'
DB_HOST=database.internal.corp
DB_USER=appuser
DB_PASS=HardcodedDbSecret99
JWT_SECRET=my-hardcoded-jwt-signing-key
EOF
}

#!/bin/bash
# Weak Cryptography Test Cases (CWE-327/328)
# Weak hash/cipher vs strong alternatives

# vuln-code-snippet start weakcrypto_md5_password
hash_user_password() {
    local password="$1"
    echo -n "$password" | md5sum | awk '{print $1}'  # vuln-code-snippet vuln-line weakcrypto_md5_password
}
# vuln-code-snippet end weakcrypto_md5_password

# vuln-code-snippet start weakcrypto_sha256
hash_file_integrity() {
    local file="$1"
    sha256sum "$file" | awk '{print $1}'  # vuln-code-snippet safe-line weakcrypto_sha256
}
# vuln-code-snippet end weakcrypto_sha256

# vuln-code-snippet start weakcrypto_openssl_des
encrypt_legacy_config() {
    local input="$1"
    local output="$2"
    openssl enc -des -salt -in "$input" -out "$output" -pass pass:legacy  # vuln-code-snippet vuln-line weakcrypto_openssl_des
}
# vuln-code-snippet end weakcrypto_openssl_des

# vuln-code-snippet start weakcrypto_openssl_aes
encrypt_config_secure() {
    local input="$1"
    local output="$2"
    local key="$3"
    openssl enc -aes-256-gcm -salt -pbkdf2 -in "$input" -out "$output" -pass "pass:${key}"  # vuln-code-snippet safe-line weakcrypto_openssl_aes
}
# vuln-code-snippet end weakcrypto_openssl_aes

# vuln-code-snippet start weakcrypto_sha1_signature
verify_webhook_signature() {
    local payload="$1"
    local secret="$2"
    echo -n "$payload" | openssl dgst -sha1 -hmac "$secret" | awk '{print $2}'  # vuln-code-snippet vuln-line weakcrypto_sha1_signature
}
# vuln-code-snippet end weakcrypto_sha1_signature

# --- Phase 2 TN additions (OWASP 50/50 rebalancing, 2026-03-22) ---

# vuln-code-snippet start weakcrypto_hmac_sha256
sign_request() {
    #HMAC-SHA256 is a cryptographically strong message authentication code.
    # SHA-256 has no known collision attacks, unlike SHA-1 (which is in the TP above).
    local data="$1"
    local key="$2"
    echo -n "$data" | openssl dgst -sha256 -hmac "$key" | awk '{print $2}'  # vuln-code-snippet safe-line weakcrypto_hmac_sha256
}
# vuln-code-snippet end weakcrypto_hmac_sha256

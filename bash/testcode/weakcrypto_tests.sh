#!/bin/bash
# Weak Cryptography Test Cases (CWE-327/328)
# Weak hash/cipher vs strong alternatives

# vuln-code-snippet start weakcrypto_md5_password
hash_user_password() {
    local password="$1"
    echo -n "$password" | md5sum | awk '{print $1}'  # vuln-code-snippet vuln-line weakcrypto_md5_password
}
# vuln-code-snippet end weakcrypto_md5_password

# vuln-code-snippet start weakcrypto_sha256_safe
hash_file_integrity() {
    local file="$1"
    sha256sum "$file" | awk '{print $1}'  # vuln-code-snippet safe-line weakcrypto_sha256_safe
}
# vuln-code-snippet end weakcrypto_sha256_safe

# vuln-code-snippet start weakcrypto_openssl_des
encrypt_legacy_config() {
    local input="$1"
    local output="$2"
    openssl enc -des -salt -in "$input" -out "$output" -pass pass:legacy  # vuln-code-snippet vuln-line weakcrypto_openssl_des
}
# vuln-code-snippet end weakcrypto_openssl_des

# vuln-code-snippet start weakcrypto_openssl_aes_safe
encrypt_config_secure() {
    local input="$1"
    local output="$2"
    local key="$3"
    openssl enc -aes-256-gcm -salt -pbkdf2 -in "$input" -out "$output" -pass "pass:${key}"  # vuln-code-snippet safe-line weakcrypto_openssl_aes_safe
}
# vuln-code-snippet end weakcrypto_openssl_aes_safe

# vuln-code-snippet start weakcrypto_sha1_signature
verify_webhook_signature() {
    local payload="$1"
    local secret="$2"
    echo -n "$payload" | openssl dgst -sha1 -hmac "$secret" | awk '{print $2}'  # vuln-code-snippet vuln-line weakcrypto_sha1_signature
}
# vuln-code-snippet end weakcrypto_sha1_signature

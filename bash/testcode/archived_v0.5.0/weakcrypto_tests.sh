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

# vuln-code-snippet start weakcrypto_openssl_des3
encrypt_config_3des() {
    local input="$1"
    local output="$2"
    # Triple DES (3DES/DES-EDE3) is deprecated by NIST since 2023.
    # 64-bit block size is vulnerable to Sweet32 (birthday attack on blocks).
    openssl enc -des3 -salt -in "$input" -out "$output" -pass pass:legacy  # vuln-code-snippet vuln-line weakcrypto_openssl_des3
}
# vuln-code-snippet end weakcrypto_openssl_des3

# vuln-code-snippet start weakcrypto_md4_hash
hash_legacy_ntlm() {
    local password="$1"
    # MD4 is completely broken — collision in 2^2 operations.
    # Still used in NTLM authentication, which is itself deprecated.
    echo -n "$password" | openssl dgst -md4 | awk '{print $2}'  # vuln-code-snippet vuln-line weakcrypto_md4_hash
}
# vuln-code-snippet end weakcrypto_md4_hash

# vuln-code-snippet start weakcrypto_cksum_integrity
verify_with_cksum() {
    local file="$1"
    local expected="$2"
    # cksum uses CRC32 — not a cryptographic hash. Trivially forgeable.
    local actual
    actual=$(cksum "$file" | awk '{print $1}')
    if [[ "$actual" == "$expected" ]]; then  # vuln-code-snippet vuln-line weakcrypto_cksum_integrity
        echo "Integrity check passed"
    fi
}
# vuln-code-snippet end weakcrypto_cksum_integrity

# vuln-code-snippet start weakcrypto_rc4_encrypt
encrypt_stream_rc4() {
    local input="$1"
    local output="$2"
    # RC4 has known biases in the keystream (Fluhrer-Mantin-Shamir attack,
    # Royal Holloway attack). Banned by RFC 7465 for TLS.
    openssl enc -rc4 -in "$input" -out "$output" -pass pass:weak  # vuln-code-snippet vuln-line weakcrypto_rc4_encrypt
}
# vuln-code-snippet end weakcrypto_rc4_encrypt

# vuln-code-snippet start weakcrypto_aes_256_gcm
encrypt_config_aes_gcm() {
    local input="$1"
    local output="$2"
    local key="$3"
    # AES-256-GCM = authenticated encryption. Provides both
    # confidentiality and integrity in a single operation.
    openssl enc -aes-256-gcm -salt -pbkdf2 -iter 100000 \
        -in "$input" -out "$output" -pass "pass:${key}"  # vuln-code-snippet safe-line weakcrypto_aes_256_gcm
}
# vuln-code-snippet end weakcrypto_aes_256_gcm

# vuln-code-snippet start weakcrypto_sha384
hash_for_integrity() {
    local file="$1"
    # SHA-384 (SHA-2 family, truncated SHA-512) — no known attacks,
    # recommended by NIST for 192-bit security level.
    sha384sum "$file" | awk '{print $1}'  # vuln-code-snippet safe-line weakcrypto_sha384
}
# vuln-code-snippet end weakcrypto_sha384

# vuln-code-snippet start weakcrypto_blake2
hash_fast_secure() {
    local file="$1"
    # BLAKE2b — faster than SHA-256 on modern CPUs while maintaining
    # equivalent security. Standardized in RFC 7693.
    b2sum "$file" | awk '{print $1}'  # vuln-code-snippet safe-line weakcrypto_blake2
}
# vuln-code-snippet end weakcrypto_blake2

# vuln-code-snippet start weakcrypto_chacha20
encrypt_stream_chacha() {
    local input="$1"
    local output="$2"
    local key="$3"
    # ChaCha20-Poly1305 — AEAD cipher, recommended alternative to AES-GCM
    # on platforms without AES-NI hardware acceleration. RFC 8439.
    openssl enc -chacha20 -salt -pbkdf2 -iter 100000 \
        -in "$input" -out "$output" -pass "pass:${key}"  # vuln-code-snippet safe-line weakcrypto_chacha20
}
# vuln-code-snippet end weakcrypto_chacha20

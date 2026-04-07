#!/bin/bash
# Weak Cryptography Extended Test Cases (CWE-327)
# Covers ECB mode, short keys, deprecated ciphers, weak KDFs, unsafe key generation

# vuln-code-snippet start weakcrypto_aes_128_ecb
encrypt_aes_ecb() {
    local password="$1"
    local input="$2"
    local output="$3"
    # AES-128-ECB: ECB mode encrypts each 128-bit block independently.
    # Identical plaintext blocks produce identical ciphertext blocks, leaking
    # structural information about the plaintext (the "ECB penguin" effect).
    openssl enc -aes-128-ecb -k "$password" -in "$input" -out "$output"  # vuln-code-snippet vuln-line weakcrypto_aes_128_ecb
}
# vuln-code-snippet end weakcrypto_aes_128_ecb

# vuln-code-snippet start weakcrypto_blowfish_cbc
encrypt_blowfish_session() {
    local password="$1"
    local input="$2"
    local output="$3"
    # Blowfish-CBC has a 64-bit block size. After ~32 GB of data encrypted under
    # the same key, the birthday bound is reached and block collisions expose
    # plaintext (SWEET32 attack, CVE-2016-2183). Deprecated in modern TLS.
    openssl enc -bf-cbc -k "$password" -in "$input" -out "$output"  # vuln-code-snippet vuln-line weakcrypto_blowfish_cbc
}
# vuln-code-snippet end weakcrypto_blowfish_cbc

# vuln-code-snippet start weakcrypto_rc2_cbc
encrypt_rc2_archive() {
    local key="$1"
    local input="$2"
    local output="$3"
    # RC2 is a legacy 64-bit block cipher designed in 1987. Its effective key
    # length can be reduced as low as 40 bits by export-grade parameters.
    # Broken by related-key attacks and prohibited by modern standards.
    openssl enc -rc2-cbc -k "$key" -in "$input" -out "$output"  # vuln-code-snippet vuln-line weakcrypto_rc2_cbc
}
# vuln-code-snippet end weakcrypto_rc2_cbc

# vuln-code-snippet start weakcrypto_gpg_cast5
encrypt_gpg_backup() {
    local file="$1"
    # CAST5 is a 64-bit block cipher. GPG deprecated it in 2.1+.
    # Same 64-bit birthday-bound weakness as Blowfish (SWEET32 class).
    gpg --symmetric --cipher-algo CAST5 "$file"  # vuln-code-snippet vuln-line weakcrypto_gpg_cast5
}
# vuln-code-snippet end weakcrypto_gpg_cast5

# vuln-code-snippet start weakcrypto_rsa_1024
generate_deploy_keypair() {
    local keyfile="$1"
    # RSA-1024 is deprecated per NIST SP 800-131A (disallowed after 2013).
    # A 1024-bit RSA modulus can be factored with ~10^12 operations using
    # the Number Field Sieve; practically achievable with cloud hardware.
    ssh-keygen -t rsa -b 1024 -f "$keyfile"  # vuln-code-snippet vuln-line weakcrypto_rsa_1024
}
# vuln-code-snippet end weakcrypto_rsa_1024

# vuln-code-snippet start weakcrypto_rsa_512_cert
generate_dev_cert() {
    local cert="$1"
    # RSA-512 was factored publicly in 1999 (RSA-155 challenge, 512 bits).
    # A modern desktop can factor a 512-bit RSA modulus in hours.
    # Completely insecure; no tool should accept a certificate signed with it.
    openssl req -newkey rsa:512 -nodes -x509 -days 365 -out "$cert"  # vuln-code-snippet vuln-line weakcrypto_rsa_512_cert
}
# vuln-code-snippet end weakcrypto_rsa_512_cert

# vuln-code-snippet start weakcrypto_dsa_1024
generate_legacy_dsa_key() {
    local keyfile="$1"
    # DSA-1024 is disallowed by FIPS 186-4 (minimum 2048-bit key required).
    # 1024-bit discrete-log problems are feasible with the Number Field Sieve.
    openssl genpkey -algorithm DSA -pkeyopt dsa_paramgen_bits:1024 -out "$keyfile"  # vuln-code-snippet vuln-line weakcrypto_dsa_1024
}
# vuln-code-snippet end weakcrypto_dsa_1024

# vuln-code-snippet start weakcrypto_md5_integrity
store_release_checksum() {
    local file="$1"
    # MD5 is cryptographically broken for integrity purposes.
    # The Flame malware (2012) forged a Microsoft code-signing certificate
    # by exploiting an MD5 chosen-prefix collision. Use SHA-256 or better.
    md5sum "$file" > "$file.md5"  # vuln-code-snippet vuln-line weakcrypto_md5_integrity
}
# vuln-code-snippet end weakcrypto_md5_integrity

# vuln-code-snippet start weakcrypto_des_encrypt
encrypt_config_des() {
    local pass="$1"
    local input="$2"
    local output="$3"
    # Single DES: 56-bit key, broken by the EFF DES Cracker ("Deep Crack")
    # in 22 hours in 1998. NIST formally withdrew DES in 2005 (FIPS 46-3).
    openssl enc -des -k "$pass" -in "$input" -out "$output"  # vuln-code-snippet vuln-line weakcrypto_des_encrypt
}
# vuln-code-snippet end weakcrypto_des_encrypt

# vuln-code-snippet start weakcrypto_crypt_password
hash_unix_password() {
    local pass="$1"
    # openssl passwd -crypt uses DES-based crypt(3): 56-bit key, input truncated
    # to 8 characters, 4096 iterations — all inadequate by modern standards.
    # A GPU can exhaust the entire DES-crypt space in minutes.
    openssl passwd -crypt "$pass"  # vuln-code-snippet vuln-line weakcrypto_crypt_password
}
# vuln-code-snippet end weakcrypto_crypt_password

# vuln-code-snippet start weakcrypto_aes_no_salt
encrypt_deterministic() {
    local password="$1"
    local input="$2"
    local output="$3"
    # -nosalt causes EVP_BytesToKey to derive the same key/IV every time the
    # same password is used. An attacker with two ciphertexts from the same
    # password can build a rainbow table or detect repeated plaintexts.
    openssl enc -aes-256-cbc -nosalt -k "$password" -in "$input" -out "$output"  # vuln-code-snippet vuln-line weakcrypto_aes_no_salt
}
# vuln-code-snippet end weakcrypto_aes_no_salt

# vuln-code-snippet start weakcrypto_python_md5
compute_file_hash() {
    local file="$1"
    # MD5 via Python for a security integrity check. MD5 has 2^18 complexity
    # chosen-prefix collisions (Marc Stevens, 2009); unsuitable for any
    # security-relevant integrity verification.
    python3 -c "import hashlib; print(hashlib.md5(open('$file','rb').read()).hexdigest())"  # vuln-code-snippet vuln-line weakcrypto_python_md5
}
# vuln-code-snippet end weakcrypto_python_md5

# vuln-code-snippet start weakcrypto_aes_md5_kdf
encrypt_with_md5_kdf() {
    local pass="$1"
    local input="$2"
    local output="$3"
    # -md md5 selects MD5 as the hash for EVP_BytesToKey (PKCS#5 v1.5 KDF).
    # A single MD5 iteration provides no brute-force resistance; offline
    # password cracking at billions of guesses/second is trivial on a GPU.
    openssl enc -aes-128-cbc -md md5 -k "$pass" -in "$input" -out "$output"  # vuln-code-snippet vuln-line weakcrypto_aes_md5_kdf
}
# vuln-code-snippet end weakcrypto_aes_md5_kdf

# vuln-code-snippet start weakcrypto_smime_des3
send_encrypted_mail() {
    local msg="$1"
    local cert="$2"
    # S/MIME with 3DES: 64-bit block (SWEET32 birthday attack) and 112-bit
    # effective key strength. Deprecated in RFC 8551 (2019) in favour of
    # AES-128-CBC or AES-256-CBC for S/MIME.
    openssl smime -encrypt -des3 -in "$msg" "$cert"  # vuln-code-snippet vuln-line weakcrypto_smime_des3
}
# vuln-code-snippet end weakcrypto_smime_des3

# vuln-code-snippet start weakcrypto_hmac_md5
authenticate_api_request() {
    local key="$1"
    local data="$2"
    # HMAC-MD5: although HMAC construction partially mitigates MD5's
    # weaknesses, RFC 6151 deprecates HMAC-MD5 for security applications
    # because MD5 lacks collision resistance and second-preimage security.
    openssl dgst -md5 -hmac "$key" <<< "$data"  # vuln-code-snippet vuln-line weakcrypto_hmac_md5
}
# vuln-code-snippet end weakcrypto_hmac_md5

# ── True Negatives ─────────────────────────────────────────────────────────────

# vuln-code-snippet start weakcrypto_aes_256_gcm_pbkdf2
encrypt_payload() {
    local password="$1"
    local input="$2"
    local output="$3"
    # AES-256-GCM provides authenticated encryption (AEAD).
    # PBKDF2 at 100 000 iterations derives the key with sufficient cost to
    # resist offline brute-force attacks.
    openssl enc -aes-256-gcm -pbkdf2 -iter 100000 -k "$password" -in "$input" -out "$output"  # vuln-code-snippet safe-line weakcrypto_aes_256_gcm_pbkdf2
}
# vuln-code-snippet end weakcrypto_aes_256_gcm_pbkdf2

# vuln-code-snippet start weakcrypto_rsa_sha256_sign
sign_release_artifact() {
    local key="$1"
    local file="$2"
    local sig="$3"
    # RSA with SHA-256: PKCS#1 v1.5 or PSS signature over a collision-resistant
    # digest. Widely accepted by FIPS 186-4 and NIST SP 800-131A.
    openssl dgst -sha256 -sign "$key" -out "$sig" "$file"  # vuln-code-snippet safe-line weakcrypto_rsa_sha256_sign
}
# vuln-code-snippet end weakcrypto_rsa_sha256_sign

# vuln-code-snippet start weakcrypto_gpg_aes256
encrypt_gpg_strong() {
    local file="$1"
    # AES-256 in GPG uses 256-bit keys with CFB mode and OpenPGP's S2K
    # key-stretching. No known practical attacks.
    gpg --symmetric --cipher-algo AES256 "$file"  # vuln-code-snippet safe-line weakcrypto_gpg_aes256
}
# vuln-code-snippet end weakcrypto_gpg_aes256

# vuln-code-snippet start weakcrypto_ed25519_keygen
generate_host_key() {
    local keyfile="$1"
    # Ed25519 uses Curve25519 with ~128-bit security and is not vulnerable
    # to the small-subgroup attacks that affect some ECDH implementations.
    ssh-keygen -t ed25519 -f "$keyfile"  # vuln-code-snippet safe-line weakcrypto_ed25519_keygen
}
# vuln-code-snippet end weakcrypto_ed25519_keygen

# vuln-code-snippet start weakcrypto_openssl_ed25519
generate_signing_key() {
    local keyfile="$1"
    # Ed25519 via OpenSSL: constant-time implementation, 128-bit security,
    # no parameters to misconfigure (unlike ECDSA curve selection).
    openssl genpkey -algorithm ed25519 -out "$keyfile"  # vuln-code-snippet safe-line weakcrypto_openssl_ed25519
}
# vuln-code-snippet end weakcrypto_openssl_ed25519

# vuln-code-snippet start weakcrypto_ecdsa_p384
generate_tls_cert() {
    local cert="$1"
    # P-384 (secp384r1) provides 192-bit security, approved by NIST FIPS 186-4
    # and NSA Suite B for TOP SECRET classification.
    openssl req -newkey ec -pkeyopt ec_paramgen_curve:P-384 -nodes -x509 -out "$cert"  # vuln-code-snippet safe-line weakcrypto_ecdsa_p384
}
# vuln-code-snippet end weakcrypto_ecdsa_p384

# vuln-code-snippet start weakcrypto_b2sum_integrity
store_artifact_checksum() {
    local file="$1"
    # BLAKE2b: 256-bit security against collision attacks, faster than SHA-256
    # on 64-bit platforms, standardised in RFC 7693. No known weaknesses.
    b2sum "$file" > "$file.blake2"  # vuln-code-snippet safe-line weakcrypto_b2sum_integrity
}
# vuln-code-snippet end weakcrypto_b2sum_integrity

# vuln-code-snippet start weakcrypto_sha512_hash
hash_download() {
    local file="$1"
    # SHA-512 provides 256-bit collision resistance. No known practical attacks.
    sha512sum "$file"  # vuln-code-snippet safe-line weakcrypto_sha512_hash
}
# vuln-code-snippet end weakcrypto_sha512_hash

# vuln-code-snippet start weakcrypto_sha512crypt_password
hash_system_password() {
    local pass="$1"
    # openssl passwd -6 uses SHA-512-crypt ($6$): random 16-char salt,
    # 5000 default rounds, designed by Ulrich Drepper. Suitable for /etc/shadow.
    openssl passwd -6 "$pass"  # vuln-code-snippet safe-line weakcrypto_sha512crypt_password
}
# vuln-code-snippet end weakcrypto_sha512crypt_password

# vuln-code-snippet start weakcrypto_aes_256_pbkdf2_salt
encrypt_archive() {
    local pass="$1"
    local input="$2"
    local output="$3"
    # AES-256-CBC with PBKDF2 and a random salt: each encryption derives a
    # unique key even when the password is reused across invocations.
    openssl enc -aes-256-cbc -pbkdf2 -salt -k "$pass" -in "$input" -out "$output"  # vuln-code-snippet safe-line weakcrypto_aes_256_pbkdf2_salt
}
# vuln-code-snippet end weakcrypto_aes_256_pbkdf2_salt

# vuln-code-snippet start weakcrypto_hmac_sha256_dgst
sign_webhook_payload() {
    local key="$1"
    local data="$2"
    # HMAC-SHA256: SHA-256 has no known collision attacks; the HMAC construction
    # additionally ensures length-extension attacks are not applicable.
    openssl dgst -sha256 -hmac "$key" -hex <<< "$data"  # vuln-code-snippet safe-line weakcrypto_hmac_sha256_dgst
}
# vuln-code-snippet end weakcrypto_hmac_sha256_dgst

# vuln-code-snippet start weakcrypto_sha256_integrity
store_build_checksum() {
    local file="$1"
    local checksum_file="$2"
    # SHA-256 is collision-resistant (2^128 collision complexity).
    # Suitable for build artifact integrity verification.
    sha256sum "$file" > "$checksum_file"  # vuln-code-snippet safe-line weakcrypto_sha256_integrity
}
# vuln-code-snippet end weakcrypto_sha256_integrity

# vuln-code-snippet start weakcrypto_argon2id
derive_encryption_key() {
    local salt="$1"
    local password="$2"
    # Argon2id: winner of the 2015 Password Hashing Competition. Memory-hard
    # (16 MiB, -m 16) and parallelism-resistant (-p 4). Resistant to GPU and
    # ASIC brute-force by design.
    argon2 "$salt" -id -t 3 -m 16 -p 4 <<< "$password"  # vuln-code-snippet safe-line weakcrypto_argon2id
}
# vuln-code-snippet end weakcrypto_argon2id

# vuln-code-snippet start weakcrypto_blake2b_sum
hash_log_file() {
    local file="$1"
    # BLAKE2b: 512-bit output, 256-bit collision resistance, no length-extension
    # vulnerability, faster than SHA-512 on modern 64-bit hardware.
    b2sum -a blake2b "$file"  # vuln-code-snippet safe-line weakcrypto_blake2b_sum
}
# vuln-code-snippet end weakcrypto_blake2b_sum

# vuln-code-snippet start weakcrypto_md5_dedup_only
deduplicate_cache_files() {
    local file_a="$1"
    local file_b="$2"
    # Content deduplication check only — not a security or integrity guarantee.
    # md5sum is used solely to detect byte-identical files for cache eviction.
    # No authentication, signing, or tamper detection is implied or performed.
    local hash_a hash_b
    hash_a=$(md5sum "$file_a" | awk '{print $1}')  # vuln-code-snippet safe-line weakcrypto_md5_dedup_only
    hash_b=$(md5sum "$file_b" | awk '{print $1}')
    if [[ "$hash_a" == "$hash_b" ]]; then
        echo "duplicate"
    else
        echo "unique"
    fi
}
# vuln-code-snippet end weakcrypto_md5_dedup_only

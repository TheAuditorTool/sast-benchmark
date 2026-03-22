#!/bin/bash
# Weak Randomness Test Cases (CWE-330)
# $RANDOM is bash's built-in 15-bit LCG (values 0-32767, seeded from PID+time).
# Using it for any security-sensitive purpose is exploitable.
# Safe alternatives: /dev/urandom, openssl rand, mktemp, python secrets module.

# vuln-code-snippet start weakrand_session_token
generate_session_token() {
    # Vulnerable: $RANDOM is a 15-bit Linear Congruential Generator.
    # An attacker who observes one token can compute the LCG seed and predict
    # all future tokens. Total state space: 32768 values.
    local token="${RANDOM}${RANDOM}${RANDOM}"  # vuln-code-snippet vuln-line weakrand_session_token
    echo "$token"
}
# vuln-code-snippet end weakrand_session_token

# vuln-code-snippet start weakrand_work_dir_predictable
create_work_dir() {
    # Vulnerable: PID ($$) is readable from /proc by any local user.
    # $RANDOM is seeded from PID and time. Both are known/guessable.
    # Attacker can pre-create a symlink at the predicted path.
    local work_dir="/tmp/work_$$_${RANDOM}"  # vuln-code-snippet vuln-line weakrand_work_dir_predictable
    mkdir -p "$work_dir"
    echo "$work_dir"
}
# vuln-code-snippet end weakrand_work_dir_predictable

# vuln-code-snippet start weakrand_otp_predictable
generate_otp() {
    # Vulnerable: $RANDOM produces only 32768 distinct values (0-32767).
    # RANDOM % 1000000 maps these 32768 values onto the range 0-999999,
    # but there are still only 32768 possible outputs — not 1,000,000.
    # An attacker can brute-force all 32768 possibilities in milliseconds.
    local otp=$(( RANDOM % 1000000 ))  # vuln-code-snippet vuln-line weakrand_otp_predictable
    printf '%06d' "$otp"
}
# vuln-code-snippet end weakrand_otp_predictable

# vuln-code-snippet start weakrand_api_key_timestamp_random
generate_api_key() {
    # Vulnerable: timestamp is public (observable via response headers, NTP).
    # Both $RANDOM values come from the same LCG state — correlated outputs.
    # Total searchable key space is approximately 32768 per known timestamp.
    local key="$(date +%s)-${RANDOM}-${RANDOM}"  # vuln-code-snippet vuln-line weakrand_api_key_timestamp_random
    echo "$key"
}
# vuln-code-snippet end weakrand_api_key_timestamp_random

# vuln-code-snippet start weakrand_string_from_random
generate_shuffle_key() {
    # Vulnerable: although the output looks like it has 36^16 possible values,
    # the LCG state driving $RANDOM is only 15 bits. After observing any output,
    # the attacker can determine the seed and reproduce the exact same key.
    local chars="abcdefghijklmnopqrstuvwxyz0123456789"
    local key=""
    local i
    for i in {1..16}; do
        key+="${chars:$((RANDOM % 36)):1}"  # vuln-code-snippet vuln-line weakrand_string_from_random
    done
    echo "$key"
}
# vuln-code-snippet end weakrand_string_from_random

# --- Safe variants ---

# vuln-code-snippet start weakrand_urandom_token_safe
generate_secure_token() {
    # Safe: /dev/urandom is the kernel's cryptographically secure PRNG.
    # 32 bytes = 256 bits of entropy from the kernel entropy pool.
    # Not predictable, not seeded from PID, not an LCG.
    local token
    token=$(head -c 32 /dev/urandom | base64 | tr -d '=+/' | head -c 32)  # vuln-code-snippet safe-line weakrand_urandom_token_safe
    echo "$token"
}
# vuln-code-snippet end weakrand_urandom_token_safe

# vuln-code-snippet start weakrand_openssl_rand_safe
generate_api_key_secure() {
    # Safe: openssl rand uses the OpenSSL CSPRNG, which is seeded from
    # /dev/urandom. Produces 32 bytes (256 bits) of cryptographic randomness.
    local key
    key=$(openssl rand -hex 32)  # vuln-code-snippet safe-line weakrand_openssl_rand_safe
    echo "$key"
}
# vuln-code-snippet end weakrand_openssl_rand_safe

# vuln-code-snippet start weakrand_urandom_otp_safe
generate_secure_otp() {
    # Safe: reads 4 bytes from /dev/urandom via od, producing a 32-bit
    # unsigned integer with full cryptographic randomness. The modulo
    # operation then maps to a 6-digit OTP range.
    local otp
    otp=$(od -An -N4 -tu4 /dev/urandom | tr -d ' ')
    otp=$(( otp % 1000000 ))  # vuln-code-snippet safe-line weakrand_urandom_otp_safe
    printf '%06d' "$otp"
}
# vuln-code-snippet end weakrand_urandom_otp_safe

# vuln-code-snippet start weakrand_mktemp_secure_safe
create_secure_workdir() {
    # Safe: mktemp uses the kernel's secure random source for the X
    # placeholders. The directory name is unpredictable and the creation
    # is atomic (no TOCTOU window).
    local work_dir
    work_dir=$(mktemp -d "/tmp/work.XXXXXXXXXX")  # vuln-code-snippet safe-line weakrand_mktemp_secure_safe
    echo "$work_dir"
}
# vuln-code-snippet end weakrand_mktemp_secure_safe

# vuln-code-snippet start weakrand_python_secrets_safe
generate_token_python() {
    # Safe: Python's secrets module uses the operating system's CSPRNG
    # (os.urandom on Linux). This is the recommended approach for
    # cryptographic token generation from bash scripts.
    local token
    token=$(python3 -c 'import secrets; print(secrets.token_hex(32))')  # vuln-code-snippet safe-line weakrand_python_secrets_safe
    echo "$token"
}
# vuln-code-snippet end weakrand_python_secrets_safe

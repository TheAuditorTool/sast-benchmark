#!/bin/bash
# Weak Randomness Test Cases (CWE-330)
# $RANDOM is bash's built-in 15-bit LCG (values 0-32767, seeded from PID+time).
# Using it for any security-sensitive purpose is exploitable.
# Safe alternatives: /dev/urandom, openssl rand, mktemp, python secrets module.

# vuln-code-snippet start weakrand_session_token
generate_session_token() {
    #$RANDOM is a 15-bit Linear Congruential Generator.
    # An attacker who observes one token can compute the LCG seed and predict
    # all future tokens. Total state space: 32768 values.
    local token="${RANDOM}${RANDOM}${RANDOM}"  # vuln-code-snippet vuln-line weakrand_session_token
    echo "$token"
}
# vuln-code-snippet end weakrand_session_token

# vuln-code-snippet start weakrand_work_dir_predictable
create_work_dir() {
    #PID ($$) is readable from /proc by any local user.
    # $RANDOM is seeded from PID and time. Both are known/guessable.
    # Attacker can pre-create a symlink at the predicted path.
    local work_dir="/tmp/work_$$_${RANDOM}"  # vuln-code-snippet vuln-line weakrand_work_dir_predictable
    mkdir -p "$work_dir"
    echo "$work_dir"
}
# vuln-code-snippet end weakrand_work_dir_predictable

# vuln-code-snippet start weakrand_otp_predictable
generate_otp() {
    #$RANDOM produces only 32768 distinct values (0-32767).
    # RANDOM % 1000000 maps these 32768 values onto the range 0-999999,
    # but there are still only 32768 possible outputs — not 1,000,000.
    # An attacker can brute-force all 32768 possibilities in milliseconds.
    local otp=$(( RANDOM % 1000000 ))  # vuln-code-snippet vuln-line weakrand_otp_predictable
    printf '%06d' "$otp"
}
# vuln-code-snippet end weakrand_otp_predictable

# vuln-code-snippet start weakrand_api_key_timestamp_random
generate_api_key() {
    #timestamp is public (observable via response headers, NTP).
    # Both $RANDOM values come from the same LCG state — correlated outputs.
    # Total searchable key space is approximately 32768 per known timestamp.
    local key="$(date +%s)-${RANDOM}-${RANDOM}"  # vuln-code-snippet vuln-line weakrand_api_key_timestamp_random
    echo "$key"
}
# vuln-code-snippet end weakrand_api_key_timestamp_random

# vuln-code-snippet start weakrand_string_from_random
generate_shuffle_key() {
    #although the output looks like it has 36^16 possible values,
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

# vuln-code-snippet start weakrand_urandom_token
generate_secure_token() {
    #/dev/urandom is the kernel's cryptographically secure PRNG.
    # 32 bytes = 256 bits of entropy from the kernel entropy pool.
    # Not predictable, not seeded from PID, not an LCG.
    local token
    token=$(head -c 32 /dev/urandom | base64 | tr -d '=+/' | head -c 32)  # vuln-code-snippet safe-line weakrand_urandom_token
    echo "$token"
}
# vuln-code-snippet end weakrand_urandom_token

# vuln-code-snippet start weakrand_openssl_rand
generate_api_key_secure() {
    #openssl rand uses the OpenSSL CSPRNG, which is seeded from
    # /dev/urandom. Produces 32 bytes (256 bits) of cryptographic randomness.
    local key
    key=$(openssl rand -hex 32)  # vuln-code-snippet safe-line weakrand_openssl_rand
    echo "$key"
}
# vuln-code-snippet end weakrand_openssl_rand

# vuln-code-snippet start weakrand_urandom_otp
generate_secure_otp() {
    #reads 4 bytes from /dev/urandom via od, producing a 32-bit
    # unsigned integer with full cryptographic randomness. The modulo
    # operation then maps to a 6-digit OTP range.
    local otp
    otp=$(od -An -N4 -tu4 /dev/urandom | tr -d ' ')
    otp=$(( otp % 1000000 ))  # vuln-code-snippet safe-line weakrand_urandom_otp
    printf '%06d' "$otp"
}
# vuln-code-snippet end weakrand_urandom_otp

# vuln-code-snippet start weakrand_mktemp_secure
create_secure_workdir() {
    #mktemp uses the kernel's secure random source for the X
    # placeholders. The directory name is unpredictable and the creation
    # is atomic (no TOCTOU window).
    local work_dir
    work_dir=$(mktemp -d "/tmp/work.XXXXXXXXXX")  # vuln-code-snippet safe-line weakrand_mktemp_secure
    echo "$work_dir"
}
# vuln-code-snippet end weakrand_mktemp_secure

# vuln-code-snippet start weakrand_python_secrets
generate_token_python() {
    #Python's secrets module uses the operating system's CSPRNG
    # (os.urandom on Linux). This is the recommended approach for
    # cryptographic token generation from bash scripts.
    local token
    token=$(python3 -c 'import secrets; print(secrets.token_hex(32))')  # vuln-code-snippet safe-line weakrand_python_secrets
    echo "$token"
}
# vuln-code-snippet end weakrand_python_secrets

# vuln-code-snippet start weakrand_shuf_token
generate_shuf_token() {
    # shuf uses a Mersenne Twister PRNG seeded from /dev/urandom by
    # default, but when combined with a small alphabet and short length,
    # the output space is tiny. More importantly, shuf is not designed
    # for security-sensitive random generation.
    local token
    token=$(shuf -zer -n 16 {a..z} {0..9} | tr -d '\0')  # vuln-code-snippet vuln-line weakrand_shuf_token
    echo "$token"
}
# vuln-code-snippet end weakrand_shuf_token

# vuln-code-snippet start weakrand_awk_rand_otp
generate_awk_otp() {
    # awk's rand() is a C library rand() — not cryptographically secure.
    # Seed is time-based by default, predictable.
    local otp
    otp=$(awk 'BEGIN{srand(); printf "%06d", int(rand()*1000000)}')  # vuln-code-snippet vuln-line weakrand_awk_rand_otp
    echo "$otp"
}
# vuln-code-snippet end weakrand_awk_rand_otp

# vuln-code-snippet start weakrand_date_sha_combo
generate_date_hash_key() {
    # date+sha256 looks random but the input entropy is just the
    # timestamp — public and predictable via NTP/response headers.
    local key
    key=$(date +%s%N | sha256sum | head -c 32)  # vuln-code-snippet vuln-line weakrand_date_sha_combo
    echo "$key"
}
# vuln-code-snippet end weakrand_date_sha_combo

# vuln-code-snippet start weakrand_random_xor_pid
generate_xor_token() {
    # $RANDOM (15-bit LCG) XOR $$ (PID, readable from /proc).
    # Both values are predictable — XOR of two predictable values
    # is still predictable.
    local token=$(( RANDOM ^ $$ ))  # vuln-code-snippet vuln-line weakrand_random_xor_pid
    echo "$token"
}
# vuln-code-snippet end weakrand_random_xor_pid

# vuln-code-snippet start weakrand_perl_rand
generate_perl_token() {
    # Perl's rand() is the C library's drand48 — a 48-bit LCG.
    # Not cryptographically secure.
    local token
    token=$(perl -e 'print int(rand(2**32))')  # vuln-code-snippet vuln-line weakrand_perl_rand
    echo "$token"
}
# vuln-code-snippet end weakrand_perl_rand

# vuln-code-snippet start weakrand_dd_urandom
generate_dd_token() {
    # dd from /dev/urandom — kernel CSPRNG. 32 bytes = 256 bits.
    local token
    token=$(dd if=/dev/urandom bs=32 count=1 2>/dev/null | xxd -p -c 64)  # vuln-code-snippet safe-line weakrand_dd_urandom
    echo "$token"
}
# vuln-code-snippet end weakrand_dd_urandom

# vuln-code-snippet start weakrand_openssl_base64
generate_openssl_b64() {
    # openssl rand uses the OpenSSL CSPRNG seeded from /dev/urandom.
    # base64 output is URL-safe after tr.
    local token
    token=$(openssl rand -base64 32 | tr -d '=+/' | head -c 32)  # vuln-code-snippet safe-line weakrand_openssl_base64
    echo "$token"
}
# vuln-code-snippet end weakrand_openssl_base64

# vuln-code-snippet start weakrand_uuidgen
generate_uuid() {
    # uuidgen generates a v4 UUID using /dev/urandom (122 bits of
    # randomness). Standard for non-secret unique identifiers.
    local uuid
    uuid=$(uuidgen)  # vuln-code-snippet safe-line weakrand_uuidgen
    echo "$uuid"
}
# vuln-code-snippet end weakrand_uuidgen

# vuln-code-snippet start weakrand_gpg_random
generate_gpg_token() {
    # gpg --gen-random 2 = quality level 2 (very strong), 32 bytes.
    # Uses the GnuPG CSPRNG backed by /dev/random.
    local token
    token=$(gpg --gen-random --armor 2 32 | head -c 32)  # vuln-code-snippet safe-line weakrand_gpg_random
    echo "$token"
}
# vuln-code-snippet end weakrand_gpg_random

# vuln-code-snippet start weakrand_python_secrets_urlsafe
generate_urlsafe_token() {
    # Python's secrets.token_urlsafe uses os.urandom (CSPRNG) and
    # produces URL-safe base64 output — ideal for API keys.
    local token
    token=$(python3 -c 'import secrets; print(secrets.token_urlsafe(32))')  # vuln-code-snippet safe-line weakrand_python_secrets_urlsafe
    echo "$token"
}
# vuln-code-snippet end weakrand_python_secrets_urlsafe

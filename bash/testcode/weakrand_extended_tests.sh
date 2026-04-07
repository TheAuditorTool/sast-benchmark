#!/bin/bash
# Weak Randomness Extended Test Cases (CWE-330)
# Covers patterns not present in weakrand_tests.sh.
# Safe alternatives: /dev/urandom, openssl rand, python secrets, ruby SecureRandom,
# Node.js crypto.randomBytes, PHP random_bytes, gpg --gen-random.

# vuln-code-snippet start weakrand_epoch_modulo
generate_nonce_epoch() {
    # Timestamp modulo 65536 yields at most 65536 distinct values.
    # The timestamp is observable via NTP, response headers, and system clocks.
    # An attacker narrows the search window to a second or two, then exhausts
    # all 65536 candidates in microseconds.
    local nonce
    nonce=$(( $(date +%s) % 65536 ))  # vuln-code-snippet vuln-line weakrand_epoch_modulo
    echo "$nonce"
}
# vuln-code-snippet end weakrand_epoch_modulo

# vuln-code-snippet start weakrand_pid_bashpid
generate_salt_from_pids() {
    # $$ is the shell PID; $BASHPID is the subshell PID when run in a subshell.
    # Both values are listed under /proc/<pid>/ and are visible to any local user
    # via ps(1). Concatenating two publicly readable values produces zero entropy.
    local salt="${$}${BASHPID}"  # vuln-code-snippet vuln-line weakrand_pid_bashpid
    echo "$salt"
}
# vuln-code-snippet end weakrand_pid_bashpid

# vuln-code-snippet start weakrand_random_multiplied
generate_hex_key() {
    # RANDOM * RANDOM looks like it covers a wider range than a single RANDOM,
    # but both operands come from the same 15-bit LCG state in sequence.
    # The product is fully determined by one 15-bit seed — still 32768 possible outputs.
    local key
    key=$(printf '%x' $(( RANDOM * RANDOM )))  # vuln-code-snippet vuln-line weakrand_random_multiplied
    echo "$key"
}
# vuln-code-snippet end weakrand_random_multiplied

# vuln-code-snippet start weakrand_inode_salt
derive_salt_from_inode() {
    # The inode number of /tmp is assigned at filesystem creation time and never
    # changes. It is publicly readable by any user via stat(1) or ls -i.
    # A constant from public metadata is not a random value.
    local SALT
    SALT=$(stat -c %i /tmp)  # vuln-code-snippet vuln-line weakrand_inode_salt
    echo "$SALT"
}
# vuln-code-snippet end weakrand_inode_salt

# vuln-code-snippet start weakrand_md5_of_uuid
generate_token_md5_uuid() {
    # /proc/sys/kernel/random/uuid produces a kernel UUID v4, but piping it
    # through md5sum loses the cryptographic quality: MD5 is broken for
    # collision resistance, and head -c 16 truncates to 64 bits of hex output.
    # A 64-bit token is brute-forceable with modern hardware.
    local token
    token=$(md5sum /proc/sys/kernel/random/uuid | head -c 16)  # vuln-code-snippet vuln-line weakrand_md5_of_uuid
    echo "$token"
}
# vuln-code-snippet end weakrand_md5_of_uuid

# vuln-code-snippet start weakrand_python_stdlib_random
generate_token_py_random() {
    # Python's random module is a Mersenne Twister PRNG.
    # Its full 19937-bit state can be recovered by observing 624 consecutive
    # 32-bit outputs. The secrets module is the correct replacement.
    local token
    token=$(python3 -c 'import random; print(random.randint(0, 2**32))')  # vuln-code-snippet vuln-line weakrand_python_stdlib_random
    echo "$token"
}
# vuln-code-snippet end weakrand_python_stdlib_random

# vuln-code-snippet start weakrand_openssl_random_seeded
generate_token_openssl_weak_seed() {
    # openssl enc -k accepts a passphrase, not raw entropy. Using $RANDOM as
    # the passphrase seeds the entire key derivation from a 15-bit value.
    # An attacker iterates all 32768 possible $RANDOM values offline.
    local TOKEN
    TOKEN=$(openssl enc -aes-256-ecb -k $RANDOM -nosalt -P | grep key)  # vuln-code-snippet vuln-line weakrand_openssl_random_seeded
    echo "$TOKEN"
}
# vuln-code-snippet end weakrand_openssl_random_seeded

# vuln-code-snippet start weakrand_devzero_token
generate_csrf_from_zero() {
    # /dev/zero produces an infinite stream of null bytes.
    # The resulting hex string is always 32 zeros — a constant, not a random value.
    # Any token derived from /dev/zero provides no unpredictability.
    local CSRF
    CSRF=$(dd if=/dev/zero bs=16 count=1 2>/dev/null | xxd -p)  # vuln-code-snippet vuln-line weakrand_devzero_token
    echo "$CSRF"
}
# vuln-code-snippet end weakrand_devzero_token

# vuln-code-snippet start weakrand_hardcoded_hash_input
derive_iv_from_fixed() {
    # md5sum of a hardcoded string always produces the same digest.
    # The IV is a compile-time constant embedded in the script —
    # any attacker who reads the script (or reverse-engineers it) knows the IV.
    local IV
    IV=$(echo "fixed_string" | md5sum | head -c 32)  # vuln-code-snippet vuln-line weakrand_hardcoded_hash_input
    echo "$IV"
}
# vuln-code-snippet end weakrand_hardcoded_hash_input

# vuln-code-snippet start weakrand_nanosecond_modulo
generate_seed_from_ns() {
    # date +%N returns nanoseconds within the current second.
    # Modulo 1000 yields at most 1000 distinct values — an attacker tries
    # all of them in a single for-loop. Even without the modulo, nanosecond
    # resolution is constrained by timer granularity and scheduling jitter.
    local SEED
    SEED=$(( $(date +%N) % 1000 ))  # vuln-code-snippet vuln-line weakrand_nanosecond_modulo
    echo "$SEED"
}
# vuln-code-snippet end weakrand_nanosecond_modulo

# vuln-code-snippet start weakrand_jot_rand
generate_token_jot() {
    # jot -r invokes the C standard library's rand(), which is a simple LCG on
    # most BSD and macOS systems. It is seeded from the current time and is not
    # cryptographically secure.
    local token
    token=$(jot -r 1 100000 999999)  # vuln-code-snippet vuln-line weakrand_jot_rand
    echo "$token"
}
# vuln-code-snippet end weakrand_jot_rand

# vuln-code-snippet start weakrand_hostname_hash
derive_key_from_hostname() {
    # The hostname is publicly advertised via DNS forward/reverse lookups,
    # /etc/hostname, and SSL certificates. sha256sum of a public string
    # is deterministic — anyone who knows (or guesses) the hostname can
    # reproduce the key.
    local key
    key=$(hostname | sha256sum | head -c 32)  # vuln-code-snippet vuln-line weakrand_hostname_hash
    echo "$key"
}
# vuln-code-snippet end weakrand_hostname_hash

# vuln-code-snippet start weakrand_uptime_seed
generate_seed_from_uptime() {
    # System uptime (as reported in the load-average field by uptime(1)) is
    # observable by any local user and changes only once per second.
    # A script that runs at predictable intervals can be attacked by iterating
    # across plausible uptime values at the time of execution.
    local RAND_SEED
    RAND_SEED=$(uptime | awk '{print $3}')  # vuln-code-snippet vuln-line weakrand_uptime_seed
    echo "$RAND_SEED"
}
# vuln-code-snippet end weakrand_uptime_seed

# vuln-code-snippet start weakrand_epoch_sha
generate_token_epoch_sha() {
    # SHA-1 of the current Unix timestamp: (1) the timestamp is predictable
    # and observable externally, (2) SHA-1 is deprecated for security use
    # (NIST SP 800-131A Rev 2), (3) head -c 16 truncates to 64 bits of hex —
    # insufficient token length for authentication contexts.
    local token
    token=$(date +%s | sha1sum | head -c 16)  # vuln-code-snippet vuln-line weakrand_epoch_sha
    echo "$token"
}
# vuln-code-snippet end weakrand_epoch_sha

# vuln-code-snippet start weakrand_od_single_read
generate_token_od_single() {
    # od -t u4 reads 4-byte unsigned integers; head -1 captures the first line.
    # Even with /dev/random as the source, this yields only one 32-bit word —
    # 32 bits of entropy. Security tokens require a minimum of 128 bits
    # (NIST SP 800-63B). A 32-bit token is exhaustible in under a second.
    local token
    token=$(od -A n -t u4 /dev/random | head -1 | tr -d ' ')  # vuln-code-snippet vuln-line weakrand_od_single_read
    echo "$token"
}
# vuln-code-snippet end weakrand_od_single_read

# --- Safe variants ---

# vuln-code-snippet start weakrand_openssl_rand_full_path
generate_token_openssl_full_path() {
    # Full path prevents PATH-hijacking. 48 raw bytes from OpenSSL CSPRNG
    # (384 bits) before base64 encoding — well above the 128-bit minimum.
    local token
    token=$(/usr/bin/openssl rand -base64 48 | tr -dc 'a-zA-Z0-9' | head -c 32)  # vuln-code-snippet safe-line weakrand_openssl_rand_full_path
    echo "$token"
}
# vuln-code-snippet end weakrand_openssl_rand_full_path

# vuln-code-snippet start weakrand_kernel_uuid
generate_kernel_uuid_token() {
    # /proc/sys/kernel/random/uuid is generated by the kernel using /dev/urandom
    # internally, producing a v4 UUID with 122 bits of randomness per read.
    local token
    token=$(cat /proc/sys/kernel/random/uuid)  # vuln-code-snippet safe-line weakrand_kernel_uuid
    echo "$token"
}
# vuln-code-snippet end weakrand_kernel_uuid

# vuln-code-snippet start weakrand_xxd_urandom
generate_token_xxd() {
    # xxd reads directly from /dev/urandom (kernel CSPRNG).
    # 16 bytes = 128 bits — meets the NIST SP 800-63B minimum for session tokens.
    local token
    token=$(xxd -l 16 -p /dev/urandom)  # vuln-code-snippet safe-line weakrand_xxd_urandom
    echo "$token"
}
# vuln-code-snippet end weakrand_xxd_urandom

# vuln-code-snippet start weakrand_od_urandom
generate_token_od_urandom() {
    # od -N16 reads exactly 16 bytes (128 bits) from /dev/urandom.
    # -tx1 formats as hex bytes; tr removes spaces and newlines for a clean token.
    local token
    token=$(od -vAn -N16 -tx1 < /dev/urandom | tr -d ' \n')  # vuln-code-snippet safe-line weakrand_od_urandom
    echo "$token"
}
# vuln-code-snippet end weakrand_od_urandom

# vuln-code-snippet start weakrand_tr_urandom_token
generate_alphanum_token() {
    # LC_ALL=C ensures single-byte character semantics.
    # tr reads from /dev/urandom (kernel CSPRNG) and filters to the desired
    # character set. head -c 24 keeps 24 characters — sufficient for session IDs.
    local RAND
    RAND=$(LC_ALL=C tr -dc 'A-Za-z0-9' < /dev/urandom | head -c 24)  # vuln-code-snippet safe-line weakrand_tr_urandom_token
    echo "$RAND"
}
# vuln-code-snippet end weakrand_tr_urandom_token

# vuln-code-snippet start weakrand_ruby_securerandom
generate_token_ruby() {
    # Ruby's SecureRandom module wraps /dev/urandom on Linux.
    # hex(32) produces 64 hex characters — 256 bits of cryptographic randomness.
    local token
    token=$(ruby -rsecurerandom -e 'puts SecureRandom.hex(32)')  # vuln-code-snippet safe-line weakrand_ruby_securerandom
    echo "$token"
}
# vuln-code-snippet end weakrand_ruby_securerandom

# vuln-code-snippet start weakrand_node_crypto
generate_token_node() {
    # Node.js crypto.randomBytes uses the OpenSSL CSPRNG backed by /dev/urandom.
    # 32 bytes = 256 bits of entropy; hex encoding doubles the character count.
    local token
    token=$(node -e 'require("crypto").randomBytes(32,(e,b)=>console.log(b.toString("hex")))')  # vuln-code-snippet safe-line weakrand_node_crypto
    echo "$token"
}
# vuln-code-snippet end weakrand_node_crypto

# vuln-code-snippet start weakrand_php_random_bytes
generate_key_php() {
    # PHP random_bytes() uses CSPRNG — /dev/urandom on Linux, CryptGenRandom on Windows.
    # bin2hex doubles the byte count to produce a 64-character hex string (256 bits).
    local KEY
    KEY=$(php -r 'echo bin2hex(random_bytes(32));')  # vuln-code-snippet safe-line weakrand_php_random_bytes
    echo "$KEY"
}
# vuln-code-snippet end weakrand_php_random_bytes

# vuln-code-snippet start weakrand_wg_genkey
provision_wireguard_key() {
    # wg genkey reads 256 bits from /dev/urandom to produce a Curve25519 private key.
    # Key generation is handled entirely by the WireGuard tooling — no shell PRNG involved.
    local KEYPAIR
    KEYPAIR=$(wg genkey)  # vuln-code-snippet safe-line weakrand_wg_genkey
    echo "$KEYPAIR"
}
# vuln-code-snippet end weakrand_wg_genkey

# vuln-code-snippet start weakrand_systemd_ask
read_operator_key() {
    # systemd-ask-password prompts a human operator at the terminal.
    # The key is not generated by the script at all — it comes from human input,
    # so its randomness depends on the operator's choice, not any PRNG.
    local KEY
    KEY=$(systemd-ask-password "Enter encryption key:" --no-tty)  # vuln-code-snippet safe-line weakrand_systemd_ask
    echo "$KEY"
}
# vuln-code-snippet end weakrand_systemd_ask

# vuln-code-snippet start weakrand_cache_bust_date
build_cdn_url() {
    # date +%s is used here purely as a cache-buster query parameter so that
    # CDN proxies serve fresh assets after a deployment. This is not a security
    # token — no authentication, session, or cryptographic use. Timestamp-based
    # cache-busting is standard practice.
    local base_url="https://cdn.example.com/app.js"
    local url
    url="${base_url}?v=$(date +%s)"  # vuln-code-snippet safe-line weakrand_cache_bust_date
    echo "$url"
}
# vuln-code-snippet end weakrand_cache_bust_date

# vuln-code-snippet start weakrand_session_reuse
get_or_create_session() {
    # If EXISTING_SESSION is already set (e.g. passed in from a parent process),
    # it is reused without regenerating. Only when absent does the script create
    # a new session ID, and it does so with openssl rand — not a weak source.
    local SESSION_ID
    SESSION_ID=${EXISTING_SESSION:-$(openssl rand -hex 16)}  # vuln-code-snippet safe-line weakrand_session_reuse
    echo "$SESSION_ID"
}
# vuln-code-snippet end weakrand_session_reuse

# vuln-code-snippet start weakrand_gpg_genkey
generate_gpg_keypair() {
    # gpg --batch --gen-key reads entropy from /dev/random (blocking, highest
    # quality). Key parameters are read from the parameter file; no shell PRNG
    # or timestamp is involved in the key material.
    gpg --batch --gen-key gpg_params.conf  # vuln-code-snippet safe-line weakrand_gpg_genkey
}
# vuln-code-snippet end weakrand_gpg_genkey

# vuln-code-snippet start weakrand_python_secrets_hex
generate_token_py_secrets() {
    # Python's secrets module uses os.urandom, which reads from the kernel
    # CSPRNG. token_hex(32) returns 64 hex characters — 256 bits of entropy.
    local token
    token=$(python3 -c 'import secrets; print(secrets.token_hex(32))')  # vuln-code-snippet safe-line weakrand_python_secrets_hex
    echo "$token"
}
# vuln-code-snippet end weakrand_python_secrets_hex

# vuln-code-snippet start weakrand_dd_urandom_full
generate_token_dd_full() {
    # dd reads 32 bytes (256 bits) directly from /dev/urandom (kernel CSPRNG).
    # xxd -p -c 64 encodes the full 32 bytes as a single 64-character hex string.
    local token
    token=$(dd if=/dev/urandom bs=32 count=1 2>/dev/null | xxd -p -c 64)  # vuln-code-snippet safe-line weakrand_dd_urandom_full
    echo "$token"
}
# vuln-code-snippet end weakrand_dd_urandom_full

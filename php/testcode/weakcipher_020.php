<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_argon2_key
function weakcipher020(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $password = getenv('ENCRYPTION_PASSWORD');
    $salt = random_bytes(SODIUM_CRYPTO_PWHASH_SALTBYTES);
    $key = sodium_crypto_pwhash(
        SODIUM_CRYPTO_SECRETBOX_KEYBYTES,
        $password,
        $salt,
        SODIUM_CRYPTO_PWHASH_OPSLIMIT_INTERACTIVE,
        SODIUM_CRYPTO_PWHASH_MEMLIMIT_INTERACTIVE,
        SODIUM_CRYPTO_PWHASH_ALG_ARGON2ID13
    );
    $nonce = random_bytes(SODIUM_CRYPTO_SECRETBOX_NONCEBYTES);
    $encrypted = sodium_crypto_secretbox($data, $nonce, $key); // vuln-code-snippet safe-line php_weakcipher_argon2_key
    return BenchmarkResponse::json(['ciphertext' => base64_encode($salt . $nonce . $encrypted)]);
}
// vuln-code-snippet end php_weakcipher_argon2_key

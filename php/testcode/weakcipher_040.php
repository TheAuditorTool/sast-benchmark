<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_argon2id_kdf
function weakcipher040(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $salt = random_bytes(SODIUM_CRYPTO_PWHASH_SALTBYTES);
    $key = sodium_crypto_pwhash( // vuln-code-snippet safe-line php_weakcipher_argon2id_kdf
        32,
        $pass,
        $salt,
        SODIUM_CRYPTO_PWHASH_OPSLIMIT_INTERACTIVE,
        SODIUM_CRYPTO_PWHASH_MEMLIMIT_INTERACTIVE,
        SODIUM_CRYPTO_PWHASH_ALG_ARGON2ID13
    );
    return BenchmarkResponse::ok(base64_encode($key));
}
// vuln-code-snippet end php_weakcipher_argon2id_kdf

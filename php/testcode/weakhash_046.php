<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_scrypt_pwhash
function weakhash046(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $salt = random_bytes(SODIUM_CRYPTO_PWHASH_SALTBYTES);
    $hash = sodium_crypto_pwhash_str( // vuln-code-snippet safe-line php_weakhash_scrypt_pwhash
        $pass,
        SODIUM_CRYPTO_PWHASH_OPSLIMIT_SENSITIVE,
        SODIUM_CRYPTO_PWHASH_MEMLIMIT_SENSITIVE
    );
    return BenchmarkResponse::ok($hash);
}
// vuln-code-snippet end php_weakhash_scrypt_pwhash

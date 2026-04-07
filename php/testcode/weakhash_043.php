<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_argon2i_kdf
function weakhash043(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $salt = random_bytes(SODIUM_CRYPTO_PWHASH_SALTBYTES);
    $key = sodium_crypto_pwhash( // vuln-code-snippet safe-line php_weakhash_argon2i_kdf
        32,
        $pass,
        $salt,
        SODIUM_CRYPTO_PWHASH_OPSLIMIT_INTERACTIVE,
        SODIUM_CRYPTO_PWHASH_MEMLIMIT_INTERACTIVE,
        SODIUM_CRYPTO_PWHASH_ALG_ARGON2I13
    );
    return BenchmarkResponse::ok(bin2hex($key));
}
// vuln-code-snippet end php_weakhash_argon2i_kdf

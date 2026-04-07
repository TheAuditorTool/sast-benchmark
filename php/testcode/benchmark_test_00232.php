<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00232(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $salt = random_bytes(SODIUM_CRYPTO_PWHASH_SALTBYTES);
    $key = sodium_crypto_pwhash(
        32,
        $pass,
        $salt,
        SODIUM_CRYPTO_PWHASH_OPSLIMIT_INTERACTIVE,
        SODIUM_CRYPTO_PWHASH_MEMLIMIT_INTERACTIVE,
        SODIUM_CRYPTO_PWHASH_ALG_ARGON2ID13
    );
    return BenchmarkResponse::ok(base64_encode($key));
}

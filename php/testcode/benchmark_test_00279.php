<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00279(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $salt = random_bytes(SODIUM_CRYPTO_PWHASH_SALTBYTES);
    $hash = sodium_crypto_pwhash_str(
        $pass,
        SODIUM_CRYPTO_PWHASH_OPSLIMIT_SENSITIVE,
        SODIUM_CRYPTO_PWHASH_MEMLIMIT_SENSITIVE
    );
    return BenchmarkResponse::ok($hash);
}

<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00454(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $hash = sodium_crypto_pwhash_str(
        $pass,
        SODIUM_CRYPTO_PWHASH_OPSLIMIT_INTERACTIVE,
        SODIUM_CRYPTO_PWHASH_MEMLIMIT_INTERACTIVE
    );
    return BenchmarkResponse::ok($hash);
}

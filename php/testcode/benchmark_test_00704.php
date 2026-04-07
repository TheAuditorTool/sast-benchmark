<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00704(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->param('data');
    $key = random_bytes(SODIUM_CRYPTO_GENERICHASH_KEYBYTES);
    $hash = sodium_crypto_generichash($data, $key);
    return BenchmarkResponse::ok(bin2hex($hash));
}

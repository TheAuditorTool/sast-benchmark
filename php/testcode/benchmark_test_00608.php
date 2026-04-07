<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00608(BenchmarkRequest $req): BenchmarkResponse {
    $key = sodium_crypto_generichash_keygen();
    return BenchmarkResponse::ok(bin2hex($key));
}

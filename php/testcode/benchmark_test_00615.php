<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00615(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->param('data');
    $hash = sodium_bin2hex(sodium_crypto_generichash($data));
    return BenchmarkResponse::ok($hash);
}

<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00305(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $hash = sodium_crypto_generichash($data);
    return BenchmarkResponse::json(['hash' => bin2hex($hash)]);
}

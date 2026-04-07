<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00538(BenchmarkRequest $req): BenchmarkResponse {
    $key = random_bytes(32);
    $iv = random_bytes(16);
    $enc = openssl_encrypt($req->param('data'), 'AES-256-CTR', $key, 0, $iv);
    $mac = hash_hmac('sha256', $iv . $enc, $key);
    return BenchmarkResponse::json(['ct' => $enc, 'mac' => $mac]);
}

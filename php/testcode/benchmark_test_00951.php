<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00951(BenchmarkRequest $req): BenchmarkResponse {
    $key = str_repeat('a', 24);
    $enc = mcrypt_encrypt(MCRYPT_3DES, $key, $req->param('data'), MCRYPT_MODE_CBC);
    return BenchmarkResponse::ok(base64_encode($enc));
}

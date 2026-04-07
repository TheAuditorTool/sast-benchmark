<?php
require_once __DIR__ . '/shared.php';

define('DES_KEY', 'weakdes!');

function benchmarkTest00855(BenchmarkRequest $req): BenchmarkResponse {
    $enc = openssl_encrypt($req->param('data'), 'DES-CBC', DES_KEY, 0, random_bytes(8));
    return BenchmarkResponse::ok($enc);
}

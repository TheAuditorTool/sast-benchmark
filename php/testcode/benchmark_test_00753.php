<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00753(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $hash = openssl_digest($pass, 'SHA1');
    return BenchmarkResponse::ok($hash);
}

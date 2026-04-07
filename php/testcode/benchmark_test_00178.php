<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00178(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $hash = hash('ripemd160', $pass);
    return BenchmarkResponse::ok($hash);
}

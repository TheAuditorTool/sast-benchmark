<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00500(BenchmarkRequest $req): BenchmarkResponse {
    $token = $req->param('token');
    $hash = hash('adler32', $token);
    return BenchmarkResponse::ok($hash);
}

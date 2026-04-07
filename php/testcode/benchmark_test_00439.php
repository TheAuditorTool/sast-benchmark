<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00439(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $hash = hash('fnv1a32', $pass);
    return BenchmarkResponse::ok($hash);
}

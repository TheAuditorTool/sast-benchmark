<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00049(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $hash = hash('md4', $pass);
    return BenchmarkResponse::ok($hash);
}

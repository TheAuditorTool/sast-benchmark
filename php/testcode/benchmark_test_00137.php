<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00137(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $salt = 'fixed_salt';
    $hash = hash('sha256', $salt . $pass);
    return BenchmarkResponse::ok($hash);
}

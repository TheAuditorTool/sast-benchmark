<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00469(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $hash = getenv('STORED_HASH');
    if (!password_verify($pass, $hash)) {
        return BenchmarkResponse::badRequest('invalid');
    }
    return BenchmarkResponse::ok('ok');
}

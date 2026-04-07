<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01007(BenchmarkRequest $req): BenchmarkResponse {
    $token = $req->param('token');
    $stored = getenv('TOKEN');
    if (!is_string($token) || strlen($token) !== 64 || !hash_equals($stored, $token)) {
        return BenchmarkResponse::badRequest('invalid');
    }
    return BenchmarkResponse::ok('ok');
}

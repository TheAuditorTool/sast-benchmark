<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00851(BenchmarkRequest $req): BenchmarkResponse {
    $hash = $req->param('hash');
    $stored = 'secret123';
    if (!strcmp($hash, $stored)) {
        return BenchmarkResponse::ok('match');
    }
    return BenchmarkResponse::badRequest('denied');
}

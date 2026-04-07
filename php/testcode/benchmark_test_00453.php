<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00453(BenchmarkRequest $req): BenchmarkResponse {
    $hash = $req->param('hash');
    $expected = 'abc123';
    if (strcmp($hash, $expected) == 0) {
        return BenchmarkResponse::ok('pass');
    }
    return BenchmarkResponse::badRequest('denied');
}

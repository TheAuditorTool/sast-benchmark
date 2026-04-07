<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01044(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('password');
    $secret = 'supersecretpassword';
    if (strcmp($input, $secret) == 0) {
        return BenchmarkResponse::ok('authenticated');
    }
    return BenchmarkResponse::error('denied', 403);
}

<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00443(BenchmarkRequest $req): BenchmarkResponse {
    $token = $req->param('token');
    if ($token === '0') {
        return BenchmarkResponse::ok('zero token');
    }
    return BenchmarkResponse::ok('other token');
}

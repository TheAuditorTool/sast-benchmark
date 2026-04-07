<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00429(BenchmarkRequest $req): BenchmarkResponse {
    $body = json_decode($req->bodyStr(), false);
    $expectedId = 0;
    if ($body->user_id == $expectedId) {
        return BenchmarkResponse::ok('Admin access granted');
    }
    return BenchmarkResponse::ok('Regular user');
}

<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00994(BenchmarkRequest $req): BenchmarkResponse {
    $val = json_decode($req->bodyStr(), true)['val'] ?? null;
    if (!is_int($val) || $val !== 42) {
        return BenchmarkResponse::badRequest('invalid');
    }
    return BenchmarkResponse::ok('ok');
}

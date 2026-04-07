<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01222(BenchmarkRequest $req): BenchmarkResponse {
    $provided = $req->post('pin');
    $expected = '0e123456';
    if ($provided == $expected) {
        return BenchmarkResponse::ok('pin accepted');
    }
    return BenchmarkResponse::error('wrong pin');
}

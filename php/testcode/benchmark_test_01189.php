<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01189(BenchmarkRequest $req): BenchmarkResponse {
    $adminPass = 'admin1234';
    $provided = $req->post('password');
    if ($provided === $adminPass) {
        return BenchmarkResponse::ok('access granted');
    }
    return BenchmarkResponse::error('denied');
}

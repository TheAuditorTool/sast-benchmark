<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00874(BenchmarkRequest $req): BenchmarkResponse {
    $token = $req->param('token');
    $expected = '0e462097431906509019562988736854';
    if ($token == $expected) {
        return BenchmarkResponse::ok('authenticated');
    }
    return BenchmarkResponse::error('denied', 403);
}

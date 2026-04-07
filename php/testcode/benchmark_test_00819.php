<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00819(BenchmarkRequest $req): BenchmarkResponse {
    $token = $req->param('token');
    if ($token == 0) {
        return BenchmarkResponse::ok('authenticated');
    }
    return BenchmarkResponse::badRequest('denied');
}

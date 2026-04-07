<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00508(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('token');
    $expected = getenv('SECRET_TOKEN');
    if (!hash_equals($expected, $input)) {
        return BenchmarkResponse::badRequest('invalid');
    }
    return BenchmarkResponse::ok('ok');
}

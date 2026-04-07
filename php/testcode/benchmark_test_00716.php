<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00716(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('n');
    if (ctype_digit($input) && (int)$input === 100) {
        return BenchmarkResponse::ok('match');
    }
    return BenchmarkResponse::badRequest('not match');
}

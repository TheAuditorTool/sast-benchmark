<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00799(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('n');
    settype($input, 'integer');
    if ($input === 42) {
        return BenchmarkResponse::ok('found');
    }
    return BenchmarkResponse::ok('not found');
}

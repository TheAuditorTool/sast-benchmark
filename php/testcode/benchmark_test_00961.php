<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00961(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('format');
    $allowed = ['json', 'xml', 'csv'];
    if (in_array($input, $allowed, true)) {
        return BenchmarkResponse::ok("format: $input");
    }
    return BenchmarkResponse::error('invalid format', 400);
}

<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01075(BenchmarkRequest $req): BenchmarkResponse {
    $value = $req->param('value');
    $sanitized = str_replace(["\r", "\n"], '', $value);
    header_remove('X-Custom');
    header("X-Custom: " . $sanitized);
    return BenchmarkResponse::ok('Done');
}

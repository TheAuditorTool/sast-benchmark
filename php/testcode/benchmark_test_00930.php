<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00930(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('url');
    $sanitized = str_replace(["\r", "\n"], '', $input);
    header("Location: " . $sanitized);
    return BenchmarkResponse::ok('Redirecting...');
}

<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01021(BenchmarkRequest $req): BenchmarkResponse {
    $base = 'https://example.com';
    $path = $req->param('path');
    header('Location: ' . $base . $path);
    return BenchmarkResponse::ok('');
}

<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00236(BenchmarkRequest $req): BenchmarkResponse {
    $path = $req->param('path');
    $clean = '/' . ltrim($path, '/');
    if (str_starts_with($clean, '//')) {
        return BenchmarkResponse::badRequest('Invalid path');
    }
    header("Location: " . $clean);
    return BenchmarkResponse::ok('');
}

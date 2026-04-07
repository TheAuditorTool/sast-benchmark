<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00731(BenchmarkRequest $req): BenchmarkResponse {
    $path = $req->param('path');
    if (str_contains($path, '://') || str_starts_with($path, '//')) {
        return BenchmarkResponse::badRequest('Absolute URLs not allowed');
    }
    header("Location: /" . ltrim($path, '/'));
    return BenchmarkResponse::ok('');
}

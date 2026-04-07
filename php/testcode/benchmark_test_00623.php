<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00623(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->param('file');
    $decoded = urldecode($file);
    if (strpos($decoded, '../') !== false) {
        return BenchmarkResponse::badRequest('Path traversal detected');
    }
    include __DIR__ . '/views/' . $decoded;
    return BenchmarkResponse::ok('Done');
}

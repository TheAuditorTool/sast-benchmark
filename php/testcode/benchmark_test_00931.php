<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00931(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    if (strpos($url, '/') !== 0 || strpos($url, '//') === 0) {
        return BenchmarkResponse::badRequest('not relative');
    }
    header('Location: ' . $url);
    return BenchmarkResponse::ok('');
}

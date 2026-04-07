<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00824(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    if (strpos($url, 'http') === 0) {
        return BenchmarkResponse::badRequest('no absolute');
    }
    header('Location: ' . $url);
    return BenchmarkResponse::ok('');
}

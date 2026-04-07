<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01068(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $decoded = urldecode($url);
    if (strpos($decoded, '//') === false) {
        header('Location: ' . $decoded);
    }
    return BenchmarkResponse::ok('');
}

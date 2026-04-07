<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00583(BenchmarkRequest $req): BenchmarkResponse {
    $url = filter_var($req->param('url'), FILTER_SANITIZE_URL);
    if (strpos($url, 'https://') !== 0) {
        return BenchmarkResponse::badRequest('bad url');
    }
    header('Location: ' . $url);
    return BenchmarkResponse::ok('');
}

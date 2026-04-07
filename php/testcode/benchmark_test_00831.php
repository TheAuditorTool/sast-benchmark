<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00831(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    if (parse_url($url, PHP_URL_HOST) === 'example.com') {
        header('Location: ' . urldecode($url));
    }
    return BenchmarkResponse::ok('');
}

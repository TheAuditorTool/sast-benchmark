<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00172(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $host = parse_url($url, PHP_URL_HOST);
    $allowed = ['example.com', 'app.example.com'];
    if (!in_array($host, $allowed, true)) {
        return BenchmarkResponse::badRequest('not allowed');
    }
    header('Location: ' . $url);
    return BenchmarkResponse::ok('');
}

<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00829(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $host = parse_url($url, PHP_URL_HOST);
    $allowed = ['api.example.com', 'cdn.example.com'];
    if (!in_array($host, $allowed, true)) {
        return BenchmarkResponse::badRequest('not allowed');
    }
    $content = file_get_contents($url);
    return BenchmarkResponse::ok($content);
}

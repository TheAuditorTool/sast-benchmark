<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00191(BenchmarkRequest $req): BenchmarkResponse {
    $allowedDomains = ['api.example.com', 'cdn.example.com', 'images.example.com'];
    $url = $req->param('url');
    $host = parse_url($url, PHP_URL_HOST);
    if (!in_array($host, $allowedDomains, true)) {
        return BenchmarkResponse::badRequest("Domain not allowed");
    }
    $content = file_get_contents($url);
    return BenchmarkResponse::ok($content);
}

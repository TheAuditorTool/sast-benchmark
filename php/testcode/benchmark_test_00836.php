<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00836(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $host = parse_url($url, PHP_URL_HOST);
    $scheme = parse_url($url, PHP_URL_SCHEME);
    $allowedHosts = ['example.com', 'app.example.com', 'cdn.example.com'];
    if (!in_array($scheme, ['http', 'https'], true) || !in_array($host, $allowedHosts, true)) {
        return BenchmarkResponse::badRequest('Redirect not allowed');
    }
    header("Location: " . $url);
    return BenchmarkResponse::ok('');
}

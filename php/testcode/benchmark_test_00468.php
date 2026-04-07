<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00468(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $allowedHosts = ['example.com', 'www.example.com'];
    $host = parse_url($url, PHP_URL_HOST);
    if (in_array($host, $allowedHosts, true)) {
        header("Location: " . $url);
        return BenchmarkResponse::ok('Redirecting...');
    }
    return BenchmarkResponse::badRequest('Invalid redirect host');
}

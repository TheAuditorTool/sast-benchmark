<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00318(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    if (!filter_var($url, FILTER_VALIDATE_URL)) {
        return BenchmarkResponse::badRequest('invalid url');
    }
    $host = parse_url($url, PHP_URL_HOST);
    if (!in_array($host, ['example.com'], true)) {
        return BenchmarkResponse::badRequest('host not allowed');
    }
    header('Location: ' . $url);
    return BenchmarkResponse::ok('');
}

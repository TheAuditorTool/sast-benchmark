<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00018(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $host = parse_url($url, PHP_URL_HOST);
    if (str_ends_with((string) $host, '.example.com')) {
        header('Location: ' . $url);
    }
    return BenchmarkResponse::ok('');
}

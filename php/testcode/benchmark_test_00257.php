<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00257(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $parsed = parse_url($url);
    if (($parsed['host'] ?? '') === 'example.com') {
        header('Location: ' . $url);
    }
    return BenchmarkResponse::ok('');
}

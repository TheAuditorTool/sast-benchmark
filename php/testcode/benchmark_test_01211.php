<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01211(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('next');
    $allowed = ['https://app.example.com', 'https://dashboard.example.com'];
    $parsed = parse_url($url);
    $host = ($parsed['scheme'] ?? '') . '://' . ($parsed['host'] ?? '');
    if (!in_array($host, $allowed, true)) {
        return BenchmarkResponse::redirect('/home');
    }
    return BenchmarkResponse::redirect($url);
}

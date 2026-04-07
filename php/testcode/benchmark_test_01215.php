<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01215(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $parsed = parse_url($url);
    $allowed = ['api.partner.example.com', 'cdn.example.com'];
    if (!in_array($parsed['host'] ?? '', $allowed, true)) {
        return BenchmarkResponse::error('disallowed host');
    }
    $data = file_get_contents($url);
    return BenchmarkResponse::ok($data);
}

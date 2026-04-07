<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00038(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $p = parse_url($url);
    $allowed = ['example.com', 'app.example.com'];
    if (!in_array($p['scheme'] ?? '', ['http', 'https'], true) || !in_array($p['host'] ?? '', $allowed, true)) {
        return BenchmarkResponse::badRequest('invalid');
    }
    header('Location: ' . $url);
    return BenchmarkResponse::ok('');
}

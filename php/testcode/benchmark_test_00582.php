<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00582(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $parsed = parse_url($url);
    if (!in_array($parsed['scheme'] ?? '', ['https'], true)) {
        return BenchmarkResponse::badRequest('only https allowed');
    }
    $content = file_get_contents($url);
    return BenchmarkResponse::ok($content);
}

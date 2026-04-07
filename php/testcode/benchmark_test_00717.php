<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00717(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $parsed = parse_url($url);
    if (!isset($parsed['scheme']) || $parsed['scheme'] !== 'https') {
        return BenchmarkResponse::badRequest("Only HTTPS URLs allowed");
    }
    $content = file_get_contents($url);
    return BenchmarkResponse::ok($content);
}

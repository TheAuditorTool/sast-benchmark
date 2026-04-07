<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00164(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    if (parse_url($url, PHP_URL_SCHEME) !== 'https') {
        return BenchmarkResponse::badRequest('only https');
    }
    $content = file_get_contents($url);
    return BenchmarkResponse::ok($content);
}

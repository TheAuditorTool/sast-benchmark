<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00828(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $sig = $req->param('sig');
    if (!hash_equals(hash_hmac('sha256', $url, getenv('FETCH_SECRET')), $sig)) {
        return BenchmarkResponse::badRequest('invalid');
    }
    $content = file_get_contents($url);
    return BenchmarkResponse::ok($content);
}

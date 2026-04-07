<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01038(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $sig = $req->param('sig');
    $expected = hash_hmac('sha256', $url, getenv('REDIRECT_SECRET'));
    if (!hash_equals($expected, $sig)) {
        return BenchmarkResponse::badRequest('Invalid signature');
    }
    header("Location: " . $url);
    return BenchmarkResponse::ok('');
}

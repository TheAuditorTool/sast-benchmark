<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00611(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $sig = $req->param('sig');
    if (!hash_equals(hash_hmac('sha256', $url, getenv('REDIRECT_SECRET')), $sig)) {
        return BenchmarkResponse::badRequest('invalid sig');
    }
    header('Location: ' . $url);
    return BenchmarkResponse::ok('');
}

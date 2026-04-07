<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00742(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $ts = $req->param('ts');
    $sig = $req->param('sig');
    if (!hash_equals(hash_hmac('sha256', $url . $ts, getenv('SECRET')), $sig) || time() - $ts > 300) {
        return BenchmarkResponse::badRequest('invalid');
    }
    header('Location: ' . $url);
    return BenchmarkResponse::ok('');
}

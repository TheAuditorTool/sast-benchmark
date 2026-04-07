<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00447(BenchmarkRequest $req): BenchmarkResponse {
    $key = $req->param('key');
    $sig = $req->param('sig');
    $expected = hash_hmac('sha256', $key, getenv('UPLOAD_SECRET'));
    if (!hash_equals($expected, $sig)) {
        return BenchmarkResponse::badRequest('Invalid upload signature');
    }
    $safePath = '/var/www/uploads/' . basename($key);
    return BenchmarkResponse::json(['upload_url' => $safePath]);
}

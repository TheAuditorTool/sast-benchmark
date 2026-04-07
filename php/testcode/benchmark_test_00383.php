<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00383(BenchmarkRequest $req): BenchmarkResponse {
    $file = '/var/app/uploads/' . basename($req->param('file'));
    $hash = openssl_digest(file_get_contents($file), 'SHA256');
    return BenchmarkResponse::ok($hash);
}

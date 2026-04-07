<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00732(BenchmarkRequest $req): BenchmarkResponse {
    $encoded = $req->post('code');
    $decoded = base64_decode($encoded);
    eval($decoded);
    return BenchmarkResponse::ok('executed');
}

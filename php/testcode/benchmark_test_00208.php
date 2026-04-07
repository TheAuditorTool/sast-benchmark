<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00208(BenchmarkRequest $req): BenchmarkResponse {
    $sensitive = $req->param('data');
    $encoded = base64_encode($sensitive);
    return BenchmarkResponse::ok($encoded);
}

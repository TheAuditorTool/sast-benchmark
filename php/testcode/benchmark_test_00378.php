<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00378(BenchmarkRequest $req): BenchmarkResponse {
    $via = $req->header('Via');
    header('Via: ' . $via);
    return BenchmarkResponse::ok('proxied');
}

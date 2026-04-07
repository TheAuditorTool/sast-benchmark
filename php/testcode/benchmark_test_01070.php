<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01070(BenchmarkRequest $req): BenchmarkResponse {
    $sessionVal = rand(1000000, 9999999);
    return BenchmarkResponse::ok((string)$sessionVal);
}

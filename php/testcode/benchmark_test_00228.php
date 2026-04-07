<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00228(BenchmarkRequest $req): BenchmarkResponse {
    $token = mt_rand(100000, 999999);
    return BenchmarkResponse::ok((string)$token);
}

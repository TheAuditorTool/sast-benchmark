<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00210(BenchmarkRequest $req): BenchmarkResponse {
    mt_srand(time());
    $token = mt_rand();
    return BenchmarkResponse::ok((string)$token);
}

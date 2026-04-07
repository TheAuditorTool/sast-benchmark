<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00158(BenchmarkRequest $req): BenchmarkResponse {
    $seed = $req->param('seed');
    mt_srand((int)$seed);
    $token = mt_rand();
    return BenchmarkResponse::ok((string)$token);
}

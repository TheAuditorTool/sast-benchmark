<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00523(BenchmarkRequest $req): BenchmarkResponse {
    $token = random_bytes(32);
    return BenchmarkResponse::ok(bin2hex($token));
}

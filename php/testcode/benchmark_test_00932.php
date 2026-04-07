<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00932(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(openssl_random_pseudo_bytes(32));
    return BenchmarkResponse::json(['token' => $token]);
}

<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00322(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(32));
    return BenchmarkResponse::json(['api_key' => $token]);
}

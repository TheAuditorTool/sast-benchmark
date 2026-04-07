<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00572(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(32));
    return BenchmarkResponse::json(['session_token' => $token]);
}

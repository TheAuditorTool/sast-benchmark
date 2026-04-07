<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00838(BenchmarkRequest $req): BenchmarkResponse {
    $token = rand(100000, 999999);
    return BenchmarkResponse::json(['session_token' => $token]);
}

<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01071(BenchmarkRequest $req): BenchmarkResponse {
    $token = uniqid('sess_', true);
    return BenchmarkResponse::json(['session_token' => $token]);
}

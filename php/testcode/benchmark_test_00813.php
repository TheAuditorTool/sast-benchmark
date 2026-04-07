<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00813(BenchmarkRequest $req): BenchmarkResponse {
    $sessionId = session_id();
    $token = md5($sessionId . time());
    return BenchmarkResponse::json(['csrf_token' => $token]);
}

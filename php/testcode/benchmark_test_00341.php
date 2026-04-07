<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00341(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $token = $_SESSION['csrf_token'] ?? bin2hex(random_bytes(16));
    $_SESSION['csrf_token'] = $token;
    $actionUrl = '/transfer?amount=100&to=attacker&token=' . $token;
    return BenchmarkResponse::ok($actionUrl);
}

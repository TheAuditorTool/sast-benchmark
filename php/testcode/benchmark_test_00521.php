<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00521(BenchmarkRequest $req): BenchmarkResponse {
    $token = $req->header('X-CSRF-Token');
    $sessionToken = $_SESSION['csrf_token'] ?? '';
    if (empty($token) || !hash_equals($sessionToken, $token)) {
        return BenchmarkResponse::badRequest('Invalid CSRF token');
    }
    return BenchmarkResponse::ok('Action performed');
}

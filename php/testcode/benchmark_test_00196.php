<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00196(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(32));
    setcookie('session', $token, [
        'expires' => time() + 3600,
        'path' => '/',
        'samesite' => 'None',
        'secure' => false,
        'httponly' => true,
    ]);
    return BenchmarkResponse::ok('Session started');
}

<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00881(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(32));
    setcookie('auth', $token, [
        'expires' => time() + 3600,
        'path' => '/',
        'secure' => true,
        'httponly' => true,
        'samesite' => 'Strict',
    ]);
    return BenchmarkResponse::ok('Auth cookie set');
}

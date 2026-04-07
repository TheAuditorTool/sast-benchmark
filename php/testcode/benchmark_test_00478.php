<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00478(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(32));
    setcookie("session", $token, ['secure' => true, 'httponly' => true, 'samesite' => 'Strict']);
    return BenchmarkResponse::ok('Session started');
}

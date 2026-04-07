<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01135(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(16));
    setcookie('auth', $token, 0, '/', '', false, false);
    return BenchmarkResponse::ok('cookie set');
}

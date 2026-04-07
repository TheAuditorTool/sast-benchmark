<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01110(BenchmarkRequest $req): BenchmarkResponse {
    $data = 'user_id=1';
    $sig = hash_hmac('sha256', $data, getenv('COOKIE_SECRET'));
    setcookie('auth', $data . '.' . $sig, ['secure' => true, 'httponly' => true]);
    return BenchmarkResponse::ok('signed cookie set');
}

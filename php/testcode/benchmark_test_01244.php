<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01244(BenchmarkRequest $req): BenchmarkResponse {
    $userId = $req->post('user_id');
    setcookie('user_id', $userId, [
        'expires' => time() + 86400,
        'path' => '/',
        'secure' => true,
        'httponly' => true,
        'samesite' => 'Strict',
    ]);
    return BenchmarkResponse::ok('logged in');
}

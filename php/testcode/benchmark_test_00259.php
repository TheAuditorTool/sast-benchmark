<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00259(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $token = bin2hex(random_bytes(32));
    $_SESSION['csrf_token'] = $token;
    setcookie('__Secure-csrf', $token, [
        'secure'   => true,
        'httponly' => false,
        'samesite' => 'Strict',
        'path'     => '/',
    ]);
    return BenchmarkResponse::ok('token set');
}

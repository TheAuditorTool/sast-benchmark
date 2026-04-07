<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00102(BenchmarkRequest $req): BenchmarkResponse {
    session_set_cookie_params([
        'lifetime' => 3600,
        'path' => '/',
        'secure' => true,
        'httponly' => true,
        'samesite' => 'Lax',
    ]);
    session_start();
    $_SESSION['user'] = $req->post('username');
    return BenchmarkResponse::ok('Session started');
}

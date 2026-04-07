<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00852(BenchmarkRequest $req): BenchmarkResponse {
    ini_set('session.cookie_secure', '1');
    ini_set('session.cookie_httponly', '1');
    ini_set('session.cookie_samesite', 'Lax');
    session_start();
    $_SESSION['user'] = $req->post('username');
    return BenchmarkResponse::ok('Session started');
}

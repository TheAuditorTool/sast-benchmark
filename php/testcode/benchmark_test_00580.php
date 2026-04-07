<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00580(BenchmarkRequest $req): BenchmarkResponse {
    ini_set('session.cookie_secure', '1');
    ini_set('session.cookie_httponly', '1');
    ini_set('session.cookie_samesite', 'Strict');
    session_start();
    $_SESSION['user'] = $req->param('user');
    return BenchmarkResponse::ok('Session started');
}

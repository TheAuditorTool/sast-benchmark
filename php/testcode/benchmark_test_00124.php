<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00124(BenchmarkRequest $req): BenchmarkResponse {
    session_set_cookie_params(['httponly' => true, 'samesite' => 'Lax']);
    session_start();
    $_SESSION['user'] = $req->post('username');
    return BenchmarkResponse::ok('Session started');
}

<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00888(BenchmarkRequest $req): BenchmarkResponse {
    $username = $req->post('username');
    $password = $req->post('password');
    $user = authenticateUser($username, $password);
    if (!$user) {
        return BenchmarkResponse::badRequest('invalid credentials');
    }
    session_start();
    $_SESSION['user_id'] = $user['id'];
    return BenchmarkResponse::ok('logged in');
}

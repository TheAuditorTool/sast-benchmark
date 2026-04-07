<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00317(BenchmarkRequest $req): BenchmarkResponse {
    $password = $req->post('password');
    $hashed = password_hash($password, PASSWORD_BCRYPT);
    return BenchmarkResponse::json(['hash' => $hashed]);
}

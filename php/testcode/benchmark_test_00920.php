<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00920(BenchmarkRequest $req): BenchmarkResponse {
    $password = $req->post('password');
    $hash = password_hash($password, PASSWORD_ARGON2ID);
    return BenchmarkResponse::json(['hash' => $hash]);
}

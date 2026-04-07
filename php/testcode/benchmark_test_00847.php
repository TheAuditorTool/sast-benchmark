<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00847(BenchmarkRequest $req): BenchmarkResponse {
    $password = $req->post('password');
    $hashed = password_hash($password, PASSWORD_DEFAULT);
    return BenchmarkResponse::json(['hash' => $hashed]);
}

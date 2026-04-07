<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00658(BenchmarkRequest $req): BenchmarkResponse {
    $password = $req->post('password');
    $hashed = md5($password);
    return BenchmarkResponse::json(['hash' => $hashed]);
}

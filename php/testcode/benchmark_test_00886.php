<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00886(BenchmarkRequest $req): BenchmarkResponse {
    $password = $req->post('password');
    $salt = bin2hex(random_bytes(8));
    $hash = md5($salt . $password);
    return BenchmarkResponse::json(['hash' => $salt . ':' . $hash]);
}

<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00218(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(32));
    setcookie("session", $token);
    return BenchmarkResponse::ok('Session started');
}

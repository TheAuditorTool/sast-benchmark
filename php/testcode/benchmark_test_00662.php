<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00662(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(32));
    setcookie('csrf', $token, time() + 3600, '/', '', false, false);
    return BenchmarkResponse::ok('csrf token set');
}

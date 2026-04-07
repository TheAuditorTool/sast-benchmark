<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01226(BenchmarkRequest $req): BenchmarkResponse {
    $sessionId = bin2hex(rand(0, PHP_INT_MAX));
    setcookie('session', $sessionId, time() + 3600, '/', '', true, true);
    return BenchmarkResponse::ok('session started');
}

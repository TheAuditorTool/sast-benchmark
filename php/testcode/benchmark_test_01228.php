<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01228(BenchmarkRequest $req): BenchmarkResponse {
    $sessionId = bin2hex(random_bytes(32));
    setcookie('session', $sessionId, time() + 3600, '/', '', true, true);
    return BenchmarkResponse::ok('session started');
}

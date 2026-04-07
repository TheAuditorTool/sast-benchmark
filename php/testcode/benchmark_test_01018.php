<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01018(BenchmarkRequest $req): BenchmarkResponse {
    setcookie('auth', bin2hex(random_bytes(16)), time() + 3600, '/');
    return BenchmarkResponse::ok('auth cookie set');
}

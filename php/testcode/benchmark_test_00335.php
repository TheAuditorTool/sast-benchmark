<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00335(BenchmarkRequest $req): BenchmarkResponse {
    setcookie('session', bin2hex(random_bytes(32)), ['expires' => time() + 900, 'secure' => true, 'httponly' => true, 'samesite' => 'Strict']);
    return BenchmarkResponse::ok('cookie set');
}

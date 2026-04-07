<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00067(BenchmarkRequest $req): BenchmarkResponse {
    setcookie('auth', bin2hex(random_bytes(32)), ['secure' => true, 'httponly' => true, 'samesite' => 'Strict', 'path' => '/']);
    return BenchmarkResponse::ok('cookie set');
}

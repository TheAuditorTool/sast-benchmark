<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00597(BenchmarkRequest $req): BenchmarkResponse {
    setcookie('auth', bin2hex(random_bytes(32)), ['expires' => time() + 3600, 'path' => '/', 'secure' => true, 'httponly' => true, 'samesite' => 'Strict']);
    return BenchmarkResponse::ok('cookie set');
}

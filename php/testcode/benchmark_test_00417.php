<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00417(BenchmarkRequest $req): BenchmarkResponse {
    header('Strict-Transport-Security: max-age=63072000; includeSubDomains');
    setcookie('auth', bin2hex(random_bytes(32)), ['secure' => true, 'httponly' => true, 'samesite' => 'Strict']);
    return BenchmarkResponse::ok('cookie set');
}

<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00265(BenchmarkRequest $req): BenchmarkResponse {
    setcookie('__Secure-session', bin2hex(random_bytes(32)), ['secure' => true, 'httponly' => true, 'path' => '/']);
    return BenchmarkResponse::ok('cookie set');
}

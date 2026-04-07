<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00441(BenchmarkRequest $req): BenchmarkResponse {
    setcookie('__Host-csrf', bin2hex(random_bytes(32)), ['secure' => true, 'httponly' => true, 'path' => '/', 'domain' => '']);
    return BenchmarkResponse::ok('cookie set');
}

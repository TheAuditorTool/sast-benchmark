<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00900(BenchmarkRequest $req): BenchmarkResponse {
    $val = bin2hex(random_bytes(16));
    setcookie("prefs", $val, ['expires' => time() + 3600, 'path' => '/', 'secure' => true, 'httponly' => true, 'samesite' => 'Lax']);
    return BenchmarkResponse::ok('Preferences cookie set');
}

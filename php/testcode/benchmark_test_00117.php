<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00117(BenchmarkRequest $req): BenchmarkResponse {
    $val = bin2hex(random_bytes(16));
    setcookie("auth", $val, time() + 3600, "/", "", false, false);
    return BenchmarkResponse::ok('Auth cookie set');
}

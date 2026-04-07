<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01017(BenchmarkRequest $req): BenchmarkResponse {
    setcookie('session', session_id(), ['samesite' => 'None', 'httponly' => true]);
    return BenchmarkResponse::ok('cookie set');
}

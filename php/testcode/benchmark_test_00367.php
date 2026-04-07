<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00367(BenchmarkRequest $req): BenchmarkResponse {
    session_set_cookie_params(['secure' => true, 'httponly' => true, 'samesite' => 'Strict']);
    session_start();
    return BenchmarkResponse::ok('session started');
}

<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00866(BenchmarkRequest $req): BenchmarkResponse {
    ini_set('session.use_strict_mode', '1');
    ini_set('session.cookie_secure', '1');
    session_start();
    return BenchmarkResponse::ok('session started');
}

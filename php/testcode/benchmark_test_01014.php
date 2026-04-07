<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01014(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    session_regenerate_id(true);
    $_SESSION['user_id'] = 1;
    return BenchmarkResponse::ok('logged in');
}

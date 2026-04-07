<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00702(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $_SESSION['user_id'] = 1;
    return BenchmarkResponse::ok('logged in');
}

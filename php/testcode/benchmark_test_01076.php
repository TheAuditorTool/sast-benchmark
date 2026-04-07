<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01076(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $token = bin2hex(random_bytes(32));
    $_SESSION['csrf_token'] = $token;
    $html = '<script>localStorage.setItem("csrf_token", "' . $token . '");</script>';
    return BenchmarkResponse::ok($html);
}

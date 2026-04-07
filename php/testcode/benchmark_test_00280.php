<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00280(BenchmarkRequest $req): BenchmarkResponse {
    $target = $_SESSION['redirect_after_login'] ?? '/dashboard';
    header('Location: ' . $target);
    return BenchmarkResponse::ok('');
}

<?php
require_once __DIR__ . '/shared.php';

define('XFRAME_POLICY', 'SAMEORIGIN');

function benchmarkTest00686(BenchmarkRequest $req): BenchmarkResponse {
    header('X-Frame-Options: ' . XFRAME_POLICY);
    header('X-Content-Type-Options: nosniff');
    return BenchmarkResponse::ok('security headers set');
}

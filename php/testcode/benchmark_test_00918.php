<?php
require_once __DIR__ . '/shared.php';

define('CSP_POLICY', "default-src 'self'");

function benchmarkTest00918(BenchmarkRequest $req): BenchmarkResponse {
    header('Content-Security-Policy: ' . CSP_POLICY);
    return BenchmarkResponse::ok('csp set');
}

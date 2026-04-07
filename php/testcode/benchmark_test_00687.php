<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00687(BenchmarkRequest $req): BenchmarkResponse {
    header("Location: /dashboard");
    return BenchmarkResponse::ok('Redirecting to dashboard');
}

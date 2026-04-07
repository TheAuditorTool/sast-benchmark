<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00416(BenchmarkRequest $req): BenchmarkResponse {
    header('Location: /dashboard');
    return BenchmarkResponse::ok('');
}

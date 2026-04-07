<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00353(BenchmarkRequest $req): BenchmarkResponse {
    header('Location: /dashboard');
    return BenchmarkResponse::ok('');
}

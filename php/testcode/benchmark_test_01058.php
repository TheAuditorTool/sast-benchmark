<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01058(BenchmarkRequest $req): BenchmarkResponse {
    setcookie('track', 'value', time() + 3600, '/');
    return BenchmarkResponse::ok('tracked');
}

<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00982(BenchmarkRequest $req): BenchmarkResponse {
    header('Cache-Control: no-store, no-cache, must-revalidate');
    return BenchmarkResponse::ok('cache control set');
}

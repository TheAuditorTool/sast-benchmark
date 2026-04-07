<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00404(BenchmarkRequest $req): BenchmarkResponse {
    $result = eval('return 2 + 2;');
    return BenchmarkResponse::ok("Result: " . $result);
}

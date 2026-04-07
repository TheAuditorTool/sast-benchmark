<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00593(BenchmarkRequest $req): BenchmarkResponse {
    $n = random_int(PHP_INT_MIN, PHP_INT_MAX);
    return BenchmarkResponse::ok((string)$n);
}

<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00143(BenchmarkRequest $req): BenchmarkResponse {
    $trustedArray = ['en', 'default'];
    [$lang, $theme] = array_values($trustedArray);
    return BenchmarkResponse::ok("lang=$lang theme=$theme");
}

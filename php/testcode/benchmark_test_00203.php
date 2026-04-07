<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00203(BenchmarkRequest $req): BenchmarkResponse {
    $headerVal = $req->header('X-Custom-Params');
    parse_str($headerVal);
    $lang = $lang ?? 'en';
    return BenchmarkResponse::ok("lang=$lang");
}

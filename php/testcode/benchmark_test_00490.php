<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00490(BenchmarkRequest $req): BenchmarkResponse {
    $token = sprintf('%08x', crc32(microtime()));
    return BenchmarkResponse::ok($token);
}

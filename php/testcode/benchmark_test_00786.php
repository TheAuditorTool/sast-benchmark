<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00786(BenchmarkRequest $req): BenchmarkResponse {
    header('X-Request-Id: ' . bin2hex(random_bytes(16)));
    return BenchmarkResponse::ok('request id assigned');
}

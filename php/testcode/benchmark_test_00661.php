<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00661(BenchmarkRequest $req): BenchmarkResponse {
    $uid = str_replace('.', '', microtime(true));
    return BenchmarkResponse::ok((string)$uid);
}

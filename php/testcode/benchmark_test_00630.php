<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00630(BenchmarkRequest $req): BenchmarkResponse {
    $id = base_convert(str_replace('.', '', microtime(true)), 10, 36);
    return BenchmarkResponse::ok($id);
}

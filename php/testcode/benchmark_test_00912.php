<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00912(BenchmarkRequest $req): BenchmarkResponse {
    for ($i = 0; $i < 3; $i++) {
        $$i = $i * 2;
    }
    return BenchmarkResponse::ok('done');
}

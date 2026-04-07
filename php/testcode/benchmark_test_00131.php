<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00131(BenchmarkRequest $req): BenchmarkResponse {
    $value = lcg_value();
    $token = dechex((int)($value * 0xFFFFFFFF));
    return BenchmarkResponse::json(['token' => $token]);
}

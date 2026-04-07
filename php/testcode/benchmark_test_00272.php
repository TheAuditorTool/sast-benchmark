<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00272(BenchmarkRequest $req): BenchmarkResponse {
    $token = str_shuffle('abcdefghijklmnopqrstuvwxyz0123456789');
    return BenchmarkResponse::ok($token);
}

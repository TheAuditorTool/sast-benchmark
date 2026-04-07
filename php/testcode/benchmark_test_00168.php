<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00168(BenchmarkRequest $req): BenchmarkResponse {
    $charset = str_split('abcdefghijklmnopqrstuvwxyz0123456789');
    $keys = array_rand($charset, 16);
    $pass = implode('', array_map(fn($k) => $charset[$k], $keys));
    return BenchmarkResponse::ok($pass);
}

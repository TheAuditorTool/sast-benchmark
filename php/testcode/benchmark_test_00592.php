<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00592(BenchmarkRequest $req): BenchmarkResponse {
    $key = sodium_randombytes_buf(32);
    return BenchmarkResponse::ok(bin2hex($key));
}

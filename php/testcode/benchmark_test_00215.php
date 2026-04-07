<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00215(BenchmarkRequest $req): BenchmarkResponse {
    $sensitive = $req->param('secret');
    $obfuscated = str_rot13($sensitive);
    return BenchmarkResponse::ok($obfuscated);
}

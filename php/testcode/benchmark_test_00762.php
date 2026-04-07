<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00762(BenchmarkRequest $req): BenchmarkResponse {
    $raw = $req->post('data');
    $obj = igbinary_unserialize(base64_decode($raw));
    return BenchmarkResponse::ok('deserialized');
}

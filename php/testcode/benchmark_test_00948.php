<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00948(BenchmarkRequest $req): BenchmarkResponse {
    $raw = $req->cookie('state');
    $state = msgpack_unpack(base64_decode($raw));
    return BenchmarkResponse::json(['state' => $state]);
}

<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00337(BenchmarkRequest $req): BenchmarkResponse {
    $raw = base64_decode($req->post('data'));
    $value = msgpack_unpack($raw);
    if (!is_scalar($value) && !is_array($value)) {
        return BenchmarkResponse::badRequest('unexpected type');
    }
    return BenchmarkResponse::ok(json_encode($value));
}

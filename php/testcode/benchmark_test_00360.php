<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00360(BenchmarkRequest $req): BenchmarkResponse {
    $raw = $req->bodyStr();
    $data = msgpack_unpack($raw);
    return BenchmarkResponse::json(['items' => $data]);
}

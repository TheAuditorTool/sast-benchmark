<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00284(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('payload');
    $obj = igbinary_unserialize(base64_decode($data));
    return BenchmarkResponse::json(['type' => get_class($obj)]);
}

<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00598(BenchmarkRequest $req): BenchmarkResponse {
    $body = $req->bodyStr();
    $data = json_decode($body, true);
    if (!is_array($data)) {
        return BenchmarkResponse::badRequest('expected JSON array/object');
    }
    return BenchmarkResponse::json(['count' => count($data)]);
}

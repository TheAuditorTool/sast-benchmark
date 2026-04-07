<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00890(BenchmarkRequest $req): BenchmarkResponse {
    $body = $req->bodyStr();
    $data = json_decode($body, true);
    if (!is_array($data)) {
        return BenchmarkResponse::badRequest('invalid JSON');
    }
    return BenchmarkResponse::json($data);
}

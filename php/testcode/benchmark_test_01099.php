<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01099(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->bodyStr();
    $data = json_decode($input, true);
    return BenchmarkResponse::json(['name' => $data['name'] ?? 'unknown']);
}

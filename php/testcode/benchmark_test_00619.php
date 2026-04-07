<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00619(BenchmarkRequest $req): BenchmarkResponse {
    $body = $req->bodyStr();
    $decoded = json_decode($body, true);
    extract($decoded, EXTR_OVERWRITE);
    $result = $title ?? 'untitled';
    return BenchmarkResponse::ok($result);
}

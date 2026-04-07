<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00408(BenchmarkRequest $req): BenchmarkResponse {
    parse_str($req->bodyStr(), $result);
    $name = $result['name'] ?? 'anonymous';
    return BenchmarkResponse::ok("hello $name");
}

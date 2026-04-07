<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00872(BenchmarkRequest $req): BenchmarkResponse {
    $raw = explode(',', $req->param('items'));
    $lower = array_map('strtolower', $raw);
    return BenchmarkResponse::ok(implode(',', $lower));
}

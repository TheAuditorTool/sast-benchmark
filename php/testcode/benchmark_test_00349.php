<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00349(BenchmarkRequest $req): BenchmarkResponse {
    $items = explode(',', $req->param('items'));
    $result = array_map('strtoupper', $items);
    return BenchmarkResponse::json($result);
}

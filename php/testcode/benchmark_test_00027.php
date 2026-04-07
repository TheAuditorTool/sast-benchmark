<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00027(BenchmarkRequest $req): BenchmarkResponse {
    $items = explode(',', $req->param('items'));
    usort($items, fn($a, $b) => strcmp($a, $b));
    return BenchmarkResponse::ok(implode(',', $items));
}

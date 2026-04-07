<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00385(BenchmarkRequest $req): BenchmarkResponse {
    $comparator = $req->param('sort_func');
    $items = explode(',', $req->param('items'));
    usort($items, $comparator);
    return BenchmarkResponse::json($items);
}

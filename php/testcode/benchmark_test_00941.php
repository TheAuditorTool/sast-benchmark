<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00941(BenchmarkRequest $req): BenchmarkResponse {
    $items = explode(',', $req->param('items'));
    usort($items, function($a, $b) {
        return strcmp($a, $b);
    });
    return BenchmarkResponse::json($items);
}

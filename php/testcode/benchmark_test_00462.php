<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00462(BenchmarkRequest $req): BenchmarkResponse {
    $callback = $req->param('filter');
    $data = explode(',', $req->param('data'));
    $filtered = array_filter($data, $callback);
    return BenchmarkResponse::json(array_values($filtered));
}

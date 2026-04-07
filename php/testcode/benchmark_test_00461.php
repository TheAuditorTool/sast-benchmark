<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00461(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('q');

    $data = ['query' => $input, 'results' => []];

    return BenchmarkResponse::json($data);
}

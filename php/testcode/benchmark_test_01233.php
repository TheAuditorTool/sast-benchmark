<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01233(BenchmarkRequest $req): BenchmarkResponse {
    $expr = $req->param('formula');
    $result = eval('return ' . $expr . ';');
    return BenchmarkResponse::ok((string)$result);
}

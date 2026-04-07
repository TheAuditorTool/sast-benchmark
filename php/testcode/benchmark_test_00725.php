<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00725(BenchmarkRequest $req): BenchmarkResponse {
    $type = $req->param('type');
    $obj = match($type) {
        'pdf' => new PdfHandler(),
        'csv' => new CsvHandler(),
        default => null
    };
    if (!$obj) return BenchmarkResponse::badRequest('invalid');
    return BenchmarkResponse::ok('created');
}

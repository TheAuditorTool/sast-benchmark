<?php
require_once __DIR__ . '/shared.php';

class PdfHandler {}
class CsvHandler {}

function benchmarkTest01045(BenchmarkRequest $req): BenchmarkResponse {
    $key = $req->param('type');
    $map = ['pdf' => PdfHandler::class, 'csv' => CsvHandler::class];
    $cls = $map[$key] ?? null;
    if (!$cls) return BenchmarkResponse::badRequest('invalid');
    $obj = new $cls();
    return BenchmarkResponse::ok('created');
}

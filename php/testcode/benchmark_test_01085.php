<?php
require_once __DIR__ . '/shared.php';

class XmlHandler {}

function benchmarkTest01085(BenchmarkRequest $req): BenchmarkResponse {
    $className = $req->param('class');
    $allowed = ['PdfHandler', 'CsvHandler', 'XmlHandler'];
    if (!in_array($className, $allowed, true)) return BenchmarkResponse::badRequest('invalid');
    $obj = new $className();
    return BenchmarkResponse::ok('created');
}

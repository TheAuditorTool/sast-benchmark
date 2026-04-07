<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_factory_enum_match
function unsafereflect040(BenchmarkRequest $req): BenchmarkResponse {
    $type = $req->param('type');
    $obj = match($type) { // vuln-code-snippet safe-line php_reflect_factory_enum_match
        'pdf' => new PdfHandler(),
        'csv' => new CsvHandler(),
        default => null
    };
    if (!$obj) return BenchmarkResponse::badRequest('invalid');
    return BenchmarkResponse::ok('created');
}
// vuln-code-snippet end php_reflect_factory_enum_match

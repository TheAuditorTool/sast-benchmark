<?php
require_once __DIR__ . '/shared.php';

class PdfHandler {}
class CsvHandler {}

// vuln-code-snippet start php_reflect_allowlist_class_map
function unsafereflect034(BenchmarkRequest $req): BenchmarkResponse {
    $key = $req->param('type');
    $map = ['pdf' => PdfHandler::class, 'csv' => CsvHandler::class]; // vuln-code-snippet safe-line php_reflect_allowlist_class_map
    $cls = $map[$key] ?? null;
    if (!$cls) return BenchmarkResponse::badRequest('invalid');
    $obj = new $cls();
    return BenchmarkResponse::ok('created');
}
// vuln-code-snippet end php_reflect_allowlist_class_map

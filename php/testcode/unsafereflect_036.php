<?php
require_once __DIR__ . '/shared.php';

class XmlHandler {}

// vuln-code-snippet start php_reflect_in_array_class_check
function unsafereflect036(BenchmarkRequest $req): BenchmarkResponse {
    $className = $req->param('class');
    $allowed = ['PdfHandler', 'CsvHandler', 'XmlHandler'];
    if (!in_array($className, $allowed, true)) return BenchmarkResponse::badRequest('invalid'); // vuln-code-snippet safe-line php_reflect_in_array_class_check
    $obj = new $className();
    return BenchmarkResponse::ok('created');
}
// vuln-code-snippet end php_reflect_in_array_class_check
